use crate::{dll::DLL, system::debug_write};
use core::mem::transmute;
use cstr_core::{cstr, CStr};

#[derive(Clone, Copy)]
pub struct Libini {
    ini_enum_sections: extern "stdcall" fn(*const u8, extern "stdcall" fn(*const u8, *const u8)),
    ini_enum_keys: extern "stdcall" fn(
        *const u8,
        *const u8,
        extern "stdcall" fn(*const u8, *const u8, *const u8, *const u8),
    ),
}

impl Libini {
    pub fn import(path: Option<&CStr>) -> Result<Self, &str> {
        let lib = DLL::init_dll(path.unwrap_or(cstr!("/sys/lib/libini.obj")));
        match lib {
            Err(e) => return Err(e),
            Ok(dll) => unsafe {
                Ok(Libini {
                    ini_enum_sections: transmute(
                        dll.get_func(cstr!("ini_enum_sections")).ok().unwrap(),
                    ),
                    ini_enum_keys: transmute(dll.get_func(cstr!("ini_enum_keys")).ok().unwrap()),
                })
            },
        }
    }

    pub fn enum_sections(
        &self,
        filename: *const u8,
        callback: extern "stdcall" fn(*const u8, *const u8),
    ) {
        debug_write("--enum_sections\n");
        (self.ini_enum_sections)(filename, callback);
    }

    pub fn enum_keys(
        self,
        filename: *const u8,
        section_name: *const u8,
        callback: extern "stdcall" fn(*const u8, *const u8, *const u8, *const u8),
    ) {
        (self.ini_enum_keys)(filename, section_name, callback);
    }
}
