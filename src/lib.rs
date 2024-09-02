use windows_sys::Win32::Graphics::Gdi::{InvalidateRect, COLOR_WINDOW, HBRUSH};
use windows_sys::Win32::Foundation::{HWND, LPARAM, LRESULT, WPARAM};
use windows_sys::Win32::System::LibraryLoader::GetModuleHandleW;
use std::os::windows::ffi::OsStrExt; use std::ffi::OsStr;
use windows_sys::Win32::UI::WindowsAndMessaging::*;
use paint::{deinitialize_gdip, initialize_gdip};
use std::ptr::null_mut;

pub mod paint;

pub fn widestr(str: &str) -> Vec<u16> { OsStr::new
(str).encode_wide().chain(Some(0).into_iter()).collect() }

#[derive(Clone, Copy)]
struct StateTransfer { noautohandle: bool, handler: EventHandler, state_ptr: isize }

unsafe extern "system" fn wnd_proc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM) -> LRESULT {
    let appstate_and_data = GetWindowLongPtrW(hwnd, GWLP_USERDATA) as *mut StateTransfer;
    if appstate_and_data.as_ref().is_none() { return DefWindowProcW(hwnd, msg, wp, lp); }
    // Automatic event handling \\
    if !(*appstate_and_data).noautohandle&&msg==WM_SIZE{InvalidateRect(hwnd,null_mut(),0);return 0;}
    if !(*appstate_and_data).noautohandle&&msg==WM_DESTROY{PostQuitMessage(0);return 0;}
    if !(*appstate_and_data).noautohandle&&msg==WM_ERASEBKGND{return 0;}
    // Automatic event handling \\
    return((*appstate_and_data).handler)(hwnd,msg,wp,lp,(*appstate_and_data).state_ptr);
}

pub type EventHandler = unsafe fn (hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM, state: isize) -> LRESULT;
pub type SetupHandler<T> = unsafe fn (hwnd: HWND, state: &mut T);

pub const fn get_y_from_lparam(lp: LPARAM) -> i16 { (lp >> 16) as i16 }
pub const fn get_x_from_lparam(lp: LPARAM) -> i16 { lp as i16 }

pub struct WindowStyles { pub class: u32, pub style: u32, pub exstyle: u32 }
impl Default for WindowStyles { fn default() -> Self { Self { class: 0, style: WS_OVERLAPPEDWINDOW | WS_VISIBLE, exstyle: 0 } } }
pub struct WindowProperties {
    pub title: String,
    pub classname: String,
    pub position: (i32, i32),
    pub style: WindowStyles,
    pub size: (i32, i32)
}

pub struct Window;
impl Window {
    pub fn new<T>(
        properties: WindowProperties,
        event_handler: EventHandler,
        setup_handler: Option<SetupHandler<T>>,
        no_auto_event_handling: bool,
        mut state: T,
    ) {
        unsafe {
            let gdiptoken = initialize_gdip();
            RegisterClassW(&WNDCLASSW {
                lpszClassName: widestr(&properties.classname).as_ptr(),
                hCursor: LoadCursorW(std::ptr::null_mut(), IDC_ARROW),
                hInstance: GetModuleHandleW(std::ptr::null_mut()),
                hbrBackground: (COLOR_WINDOW + 1) as HBRUSH,
                lpszMenuName: std::ptr::null_mut(),
                style: properties.style.class,
                cbClsExtra: 0, cbWndExtra: 0,
                hIcon: std::ptr::null_mut(),
                lpfnWndProc: Some(wnd_proc)
            });
            let hwnd = CreateWindowExW(
                properties.style.exstyle,
                widestr(&properties.classname).as_ptr(),
                widestr(&properties.title).as_ptr(),
                properties.style.style,
                properties.position.0, 
                properties.position.1,
                properties.size.0, 
                properties.size.1,
                std::ptr::null_mut(), std::ptr::null_mut(),
                GetModuleHandleW(std::ptr::null_mut()),
                std::ptr::null_mut()
            );
            if let Some(setup_handler) = setup_handler { setup_handler(hwnd, &mut state); }
            let mut state = StateTransfer { noautohandle: no_auto_event_handling,
                handler: event_handler, state_ptr: &mut state as *mut T as isize };
            SetWindowLongPtrW(hwnd, GWLP_USERDATA, &mut state as *mut StateTransfer as isize);
            let mut msg: MSG = std::mem::zeroed();
            while GetMessageW(&mut msg, hwnd, 0, 0) > 0
            { TranslateMessage(&msg); DispatchMessageW(&msg); }
            deinitialize_gdip(gdiptoken);
        };
    }
}