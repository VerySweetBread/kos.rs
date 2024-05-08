use crate::dll::DLL;
use core::ffi::CStr;
use core::mem::transmute;

pub struct Console {
    con_init: extern "stdcall" fn(u32, u32, u32, u32, *const u8),
    con_write_string: extern "stdcall" fn(*const u8, u32),
    con_exit: extern "stdcall" fn(bool),
}

impl Console {
    pub fn import(path: Option<&CStr>) -> Result<Self, &str> {
        let lib = DLL::load_dll(path.unwrap_or(c"/sys/lib/console.obj"));
        match lib {
            Err(e) => return Err(e),
            Ok(dll) => unsafe {
                Ok(Console {
                    con_init: transmute(dll.get_func(c"con_init").ok().unwrap()),
                    con_write_string: transmute(dll.get_func(c"con_write_string").ok().unwrap()),
                    con_exit: transmute(dll.get_func(c"con_exit").ok().unwrap()),
                })
            },
        }
    }

    pub fn init(&self, x: u32, y: u32, width: u32, height: u32, title: &CStr) {
        (self.con_init)(x, y, width, height, title.as_ptr() as *const u8);
    }

    pub fn write_string(&self, text: &str) {
        (self.con_write_string)(text.as_ptr(), text.len() as u32);
    }

    pub fn exit(self, close_window: bool) {
        (self.con_exit)(close_window);
    }
}
