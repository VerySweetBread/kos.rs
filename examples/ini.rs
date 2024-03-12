#![no_std]
#![no_main]

use cstr_core::{cstr, CStr};
use kos::{dll::Libini, system::debug_write, threads::exit};

extern crate alloc;
use alloc::format;

static mut LIB: Option<Libini> = None;

#[no_mangle]
pub fn kol_main() {
    unsafe {
        // FIXME: Find any possible solution to drop that fucking unsafe
        LIB = Libini::import(None).ok();
        let filename = cstr!("/sys/File Managers/Kfar.ini");
        LIB.unwrap()
            .enum_sections(filename.as_ptr() as *const u8, sec_handler);
    }

    exit();
}

extern "stdcall" fn sec_handler(filename: *const u8, section: *const u8) {
    unsafe {
        debug_write(&format!(
            "{}\n",
            CStr::from_ptr(section as *const i8).to_str().unwrap()
        ));

        LIB.unwrap().enum_keys(filename, section, key_handler);
    };
}

extern "stdcall" fn key_handler(
    _filename: *const u8,
    _section: *const u8,
    key: *const u8,
    value: *const u8,
) {
    unsafe {
        debug_write(&format!(
            "- {}\t = {}\n",
            CStr::from_ptr(key as *const i8).to_str().unwrap(),
            CStr::from_ptr(value as *const i8).to_str().unwrap()
        ))
    }
}
