#![no_std]
#![no_main]

use cstr_core::{cstr, CStr};

use kos::{
    graphics::{display_message, Color, Dot, Size},
    input::fetch_key,
    system::{get_lang, Lang},
    threads::{exit, fetch_event, Event},
    windows::{
        define_button, define_window, end_window_draw, get_button_id, start_window_draw,
        WindowKind, WindowParams, CLOSE_BUTTON,
    },
};

const HEADER: &CStr = cstr!("Hey Kolibri");
const MSG: &CStr = cstr!("Hello from Rust!");
const BTN: u32 = 42;

#[macro_use]
extern crate alloc;

fn draw_window(c: usize) {
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

    display_message(
        Dot { x: 10, y: 10 },
        Color::rgb(0x66, 0x22, 0x22),
        MSG,
        None,
    );

    let btn_str = match get_lang() {
        Lang::German => format!("Taste gedrückt: {} mal\0", c),
        Lang::Russian => format!("Кнопка нажата: {} раз\0", c),
        Lang::French => format!("Button enfoncé : {} fois\0", c),
        _ => format!("Button pressed: {} times\0", c),
    };

    display_message(
        Dot { x: 10, y: 30 },
        Color::rgb(0, 0, 0),
        CStr::from_bytes_with_nul(btn_str.as_bytes()).unwrap_or(cstr!("String error")),
        None,
    );

    define_button(
        Dot { x: 10, y: 70 },
        Size {
            width: 70,
            height: 15,
        },
        BTN,
        true,
        true,
        Some(Color::rgb(128, 255, 128)),
    );

    end_window_draw();

    return;
}

fn button_handler(c: &mut usize) {
    let btn_id = get_button_id();

    if btn_id.is_some() {
        match btn_id.unwrap() {
            CLOSE_BUTTON => exit(),
            BTN => {
                *c += 1;
                draw_window(*c);
            }
            _ => {}
        }
    }
}

#[no_mangle]
fn kol_main() {
    let mut c = 0;

    while let Some(ev) = fetch_event() {
        match ev {
            Event::Redraw => draw_window(c),
            Event::KeyPress => drop(fetch_key()),
            Event::BtnPress => button_handler(&mut c),
            _ => break,
        }
    }
}
