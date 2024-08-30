use windows_sys::Win32::Graphics::GdiPlus::*;
use windows_sys::Win32::Graphics::Gdi::*;
use windows_sys::Win32::Foundation::HWND;
use std::os::raw::c_void;

pub struct PaintData {
    pub graphics: *mut GpGraphics,
    hdc_screen: *mut c_void,
    hdc_mem: *mut c_void,
    hbm_old: *mut c_void,
    pub ps: PAINTSTRUCT,
    hwnd: HWND
}

pub(crate) fn initialize_gdip() -> usize {
    let mut gdiplus_token = 0;
    let startup_input = GdiplusStartupInput {
        SuppressExternalCodecs: 0, DebugEventCallback: 0, SuppressBackgroundThread: 0, GdiplusVersion: 1
    }; let mut startup_output = GdiplusStartupOutput { NotificationHook: 0, NotificationUnhook: 0 };
    unsafe { GdiplusStartup(&mut gdiplus_token, &startup_input, &mut startup_output) };
    gdiplus_token
}
pub(crate) fn deinitialize_gdip(token: usize) { unsafe { GdiplusShutdown(token); } }

pub fn init_paint(hwnd: HWND) -> PaintData {
    unsafe {
        let mut ps: PAINTSTRUCT = std::mem::zeroed();
        let hdc_screen = BeginPaint(hwnd, &mut ps);
        let hdc_mem = CreateCompatibleDC(hdc_screen);
        let hbm_mem = CreateCompatibleBitmap(hdc_screen, 
            (ps.rcPaint.right - ps.rcPaint.left) as i32, 
            (ps.rcPaint.bottom - ps.rcPaint.top) as i32);
        let hbm_old = SelectObject(hdc_mem, hbm_mem);
        let mut graphics: *mut GpGraphics = std::ptr::null_mut();
        GdipCreateFromHDC(hdc_mem, &mut graphics);
        PaintData {
            hdc_screen, graphics, hdc_mem,
            hbm_old, hwnd, ps,
        }
    }
    
}
pub fn cleanup_paint(pdata: PaintData) {
    unsafe {
        BitBlt(pdata.hdc_screen, pdata.ps.rcPaint.left, pdata.ps.rcPaint.top,
            (pdata.ps.rcPaint.right - pdata.ps.rcPaint.left) as i32,
            (pdata.ps.rcPaint.bottom - pdata.ps.rcPaint.top) as i32,
            pdata.hdc_mem, pdata.ps.rcPaint.left, pdata.ps.rcPaint.top, SRCCOPY);
        GdipDeleteGraphics(pdata.graphics);
        DeleteObject(SelectObject(pdata.hdc_mem, pdata.hbm_old));
        DeleteDC(pdata.hdc_mem);
        EndPaint(pdata.hwnd, &pdata.ps);
    }
}