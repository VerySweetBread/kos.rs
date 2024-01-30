#![no_std]
#![no_main]

use cstr_core::cstr;
use kos::{dll::Console, threads::exit};

extern crate alloc;

#[no_mangle]
pub fn kol_main() {
    let header = cstr!("Rust!");
    let string = "Hi from Rust!";

    let con_lib = Console::import(None).unwrap();
    con_lib.init(u32::MAX, u32::MAX, u32::MAX, u32::MAX, header);
    con_lib.write_string(string);
    con_lib.exit(false);

    exit();
}
