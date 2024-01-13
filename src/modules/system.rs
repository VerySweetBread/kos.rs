use crate::sys;
use alloc::string::String;
use cstr_core::CStr;

trait Debuggable {
    fn data_iter(self) -> impl Iterator<Item = u8>;
}

impl Debuggable for &str {
    fn data_iter(self) -> impl Iterator<Item = u8> {
        self.bytes()
    }
}

impl Debuggable for &String {
    fn data_iter(self) -> impl Iterator<Item = u8> {
        self.as_bytes().iter().copied()
    }
}

impl Debuggable for &CStr {
    fn data_iter(self) -> impl Iterator<Item = u8> {
        self.to_bytes().iter().copied()
    }
}

pub fn debug_write<Str: Debuggable>(text: Str) {
    for byte in text.data_iter() {
        unsafe {
            sys::_debug_write(byte);
        }
    }
}
