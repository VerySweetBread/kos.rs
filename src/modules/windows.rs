use crate::graphics::{Color, Dot, Size};
use crate::sys;
use crate::system::debug_write;
use crate::throw_new;
use cstr_core::{cstr, CStr};

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

pub const CLOSE_BUTTON: u32 = 1;

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

pub fn define_button(
    start: Dot,
    size: Size,
    id: u32,
    draw: bool,
    border: bool,
    color: Option<Color>,
) {
    if 0 >= size.width || size.width >= 0x8000 || 0 >= size.height || size.height >= 0x8000 {
        crate::graphics::display_message(
            Dot { x: 10, y: 200 },
            Color::rgb(255, 0, 0),
            CStr::from_bytes_with_nul(
                format!(
                    "x:{:?} y:{:?} w:{:?} h:{:?}\n\0",
                    start.x, start.y, size.width, size.height
                )
                .as_bytes(),
            )
            .unwrap_or(cstr!("String error")),
            None,
        );
        throw_new!(format!(
            "x:{:?} y:{:?} w:{:?} h:{:?}\n",
            start.x, start.y, size.width, size.height
        ));
        return;
    }
    if id > 0xFFFFFF {
        throw_new!("Invalid button ID");
        return;
    }

    let mut flags = 0;
    if !draw {
        flags += 1 << 30
    };
    if !border {
        flags += 1 << 29
    };

    unsafe {
        sys::define_button(
            start.x << 16 | size.width,
            start.y << 16 | size.height,
            flags << 29 | id,
            color.unwrap_or(Color::rgb(255, 255, 255)).as_rgb_val(),
        );
    }
}

// TODO: mouse button info
pub fn get_button_id() -> Option<u32> {
    unsafe {
        let eax = sys::get_button_id();
        if eax == 1 {
            return None;
        }
        return Some(eax >> 8);
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
