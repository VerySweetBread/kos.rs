#![no_std]
#![no_main]

use cstr_core::{cstr, CStr};

use kos::graphics::{display_message, Color, Dot, Size};
use kos::input::fetch_key;
use kos::threads::{exit, fetch_event, Event};
use kos::windows::{define_window, end_window_draw, start_window_draw, WindowKind, WindowParams};

const HEADER: &CStr = cstr!("Hey Kolibri");
const MSG: &CStr = cstr!("Hello from Rust!");

#[inline(always)] // for some reason function removed otherwise
fn draw_window() {
    start_window_draw();
    define_window(
        Dot { x: 50, y: 50 },
        Size {
            width: 300,
            height: 400,
        },
        WindowParams {
            color: Color::rgb(0xff, 0xff, 0xff),
            kind: WindowKind::Themed,
            title: Some(HEADER),
        },
    );
    display_message(Dot { x: 0, y: 10 }, Color::rgb(0x66, 0x22, 0x22), MSG, None);
    end_window_draw();
}

#[no_mangle]
fn kol_main() -> ! {
    draw_window();

    while let Some(ev) = fetch_event() {
        match ev {
            Event::Redraw => draw_window(),
            Event::KeyPress => drop(fetch_key()),
            _ => break,
        }
    }

    exit();
}
