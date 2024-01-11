#![no_std]

mod nanolibc;
mod sys;

pub use sys::*;

#[derive(Clone, Copy)]
pub struct Color(u8, u8, u8);

impl Color {
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self(r, g, b)
    }

    pub fn r(&self) -> u8 {
        self.0
    }

    pub fn g(&self) -> u8 {
        self.1
    }

    pub fn b(&self) -> u8 {
        self.2
    }

    pub fn as_rgb_val(self) -> u32 {
        (self.0 as u32) << 16 | (self.1 as u32) << 8 | (self.2 as u32)
    }
}

pub struct Dot {
    pub x: u32,
    pub y: u32,
}

pub struct Size {
    pub width: u32,
    pub height: u32,
}

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

pub struct WindowTextParams<'a> {
    pub color: Color,
    pub text: &'a str,
    pub bg_color: Option<Color>,
}

pub fn display_message(start: Dot, params: WindowTextParams<'_>) {
    const UTF8_FLAG: u32 = 0b0011_0000 << 24;
    const BG_FLAG: u32 = 0b0100_0000 << 24;

    unsafe {
        sys::display_message(
            start.x << 16 | start.y,
            params.color.as_rgb_val() | BG_FLAG * params.bg_color.is_some() as u32 | UTF8_FLAG,
            params.text.as_ptr() as u32,
            params.text.len() as u32,
            0,
        );
    }
}

pub fn exit() -> ! {
    unsafe { sys::exit() }
}

pub fn start_window_draw() {
    unsafe { sys::start_window_draw() }
}

pub fn end_window_draw() {
    unsafe { sys::end_window_draw() }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    exit();
}

#[non_exhaustive]
pub enum Event {
    Redraw,
    KeyPress,
}

pub fn fetch_event() -> Option<Event> {
    match unsafe { sys::wait_event() } {
        1 => Some(Event::Redraw),
        2 => Some(Event::KeyPress),
        _ => None,
    }
}

pub fn fetch_key() -> Option<u8> {
    let res = unsafe { sys::pressed_key() };
    if res == 1 {
        None
    } else {
        Some(((res >> 8) & 0xff) as u8)
    }
}
