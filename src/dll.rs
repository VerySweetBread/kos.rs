use crate::sys;
use core::ffi::CStr;

mod console;
pub use console::Console;

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

    pub fn get_func(&self, name: &CStr) -> Result<*const (), &str> {
        unsafe {
            let mut i = 0;
            loop {
                let el = self.table.add(i);
                if el as usize == 0 {
                    return Err("Function not found");
                }
                let cur_name = CStr::from_ptr((*el).func_name as *const i8);
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
