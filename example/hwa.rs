#![no_std]
#![no_main]

use cstr_core::{cstr, CStr};

use kos::{Color, Dot, Event, WindowKind, WindowParams, WindowTextParams};

const HEADER: &CStr = cstr!("Hey Kolibri");
const MSG: &str = "Hello from Rust!";

#[inline(always)] // for some reason function removed otherwise
fn draw_window() {
    unsafe {
        kos::start_window_draw();
        kos::define_window(
            Dot { x: 50, y: 50 },
            300, 400,
            WindowParams {
                color: Color::rgb(0xff, 0xff, 0xff),
                kind: WindowKind::Themed,
                title: Some(HEADER),
            },
        );
        kos::display_message(
            Dot { x: 0, y: 10 },
            WindowTextParams {
                color: Color::rgb(0x66, 0x22, 0x22),
                text: MSG,
                bg_color: None,
            },
        );
        kos::end_window_draw();
    }
}

#[no_mangle]
fn kol_main() -> ! {
    draw_window();

    while let Some(ev) = kos::fetch_event() {
        match ev {
            Event::Redraw => draw_window(),
            Event::KeyPress => drop(kos::fetch_key()),
            _ => break,
        }
    }

    kos::exit();
}
