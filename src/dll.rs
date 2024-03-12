use crate::sys;
use core::mem::transmute;
use cstr_core::{cstr, CStr};

mod console;
pub use console::Console;

mod libini;
pub use libini::Libini;

use crate::system::debug_write;

pub struct DLL {
    table: *const ImportTableEl,
}

impl DLL {
    pub fn load_dll(name: &CStr) -> Result<DLL, &str> {
        unsafe {
            let table = sys::load_dll(name.as_ptr() as *const u8);
            if table as usize == 0 {
                return Err("Library load error");
            }

            return Ok(DLL {
                table: table as *const ImportTableEl,
            });
        }
    }

    pub fn init_dll(name: &CStr) -> Result<DLL, &str> {
        unsafe {
            let lib = DLL::load_dll(name);
            match lib {
                Err(e) => Err(e),
                Ok(dll) => {
                    let init: fn() = transmute(dll.get_func(cstr!("lib_init")).ok().unwrap());
                    sys::dllInit(init);
                    Ok(dll)
                }
            }
        }
    }

    pub fn get_func(&self, name: &CStr) -> Result<*const (), &str> {
        unsafe {
            let mut i = 0;
            loop {
                let el = self.table.add(i);
                if el as usize == 0 {
                    return Err("Function not found");
                }
                let cur_name = CStr::from_ptr((*el).func_name as *const i8);
                debug_write(&format!(
                    "{}: {:x}\n",
                    cur_name.to_str().unwrap(),
                    (*el).func_addr as usize
                ));
                if cur_name == name {
                    return Ok((*el).func_addr as *const ());
                }
                i += 1;
            }
        }
    }
}
#[repr(C)] // Avoid random field order
struct ImportTableEl {
    pub func_name: *const u8,
    pub func_addr: fn(),
}
