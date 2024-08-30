# llwlib - A costless windows api wrapper to create windows

### To create a window:
```rust
use llwlib::{Window, WindowProperties, WindowStyles};
use windows_sys::Win32::UI::WindowsAndMessaging::*;
use windows_sys::Win32::Foundation::*;

fn main() {
    Window::new(
        WindowProperties {
            style: WindowStyles { ..Default::default() },
            position: (CW_USEDEFAULT, CW_USEDEFAULT),
            classname: "class_name".into(),
            title: "Title".into(),
            size: (1200, 700)
        },
        wnd_proc,
        false,
        ()
    );
}

unsafe fn wnd_proc(hwnd: HWND, msg: u32, wp: WPARAM, lp: LPARAM, _: isize) -> LRESULT {
    DefWindowProcW(hwnd, msg, wp, lp)
}
```
