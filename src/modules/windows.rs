use crate::graphics::{Color, Dot, Size};
use crate::sys;

#[repr(u32)]
pub enum WindowKind {
    Fixed = 0,
    NoDraw = 1,
    Resizable = 2,
    Themed = 3,
    FixedThemed = 4,
}

pub struct WindowParams<'a> {
    pub color: Color,
    pub kind: WindowKind,
    pub title: Option<&'a cstr_core::CStr>,
}

pub fn define_window(start: Dot, size: Size, params: WindowParams<'_>) {
    const RELATIVE_FLAG: u32 = 0x20;

    unsafe {
        sys::define_window(
            start.x << 16 | size.width,
            start.y << 16 | size.height,
            params.color.as_rgb_val()
                | (RELATIVE_FLAG | (params.title.is_some() as u32) << 4 | params.kind as u32) << 24,
            0,
            params
                .title
                .map(|s| s.as_ptr())
                .unwrap_or_else(core::ptr::null) as u32,
        );
    }
}

pub fn start_window_draw() {
    unsafe { sys::start_window_draw() }
}

pub fn end_window_draw() {
    unsafe { sys::end_window_draw() }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe {
        sys::exit();
    }
}
