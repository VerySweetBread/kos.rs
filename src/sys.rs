#[link(name = "syscalls")]
extern "C" {
    // -1
    #[link_name = "_exit"]
    pub fn exit() -> !;

    // 0
    #[link_name = "_define_window"]
    pub fn define_window(ebx: u32, ecx: u32, edx: u32, esi: u32, edi: u32);

    // 2
    #[link_name = "_pressed_key"]
    pub fn pressed_key() -> u32;

    // 4
    #[link_name = "_display_message"]
    pub fn display_message(ebx: u32, ecx: u32, edx: u32, edi: u32);

    // 8
    #[link_name = "_define_button"]
    pub fn define_button(ebx: u32, ecx: u32, edx: u32, esi: u32);

    // 10
    #[link_name = "_wait_event"]
    pub fn wait_event() -> u32;

    // 12.1
    #[link_name = "_start_window_draw"]
    pub fn start_window_draw();

    // 12.2
    #[link_name = "_end_window_draw"]
    pub fn end_window_draw();

    // 17
    #[link_name = "_get_button_id"]
    pub fn get_button_id() -> u32;

    // 26.5
    #[link_name = "_get_lang"]
    pub fn get_lang() -> u32;

    // 63.1
    #[link_name = "_debug_write"]
    pub fn _debug_write(cl: u8);

    // 68.11
    #[link_name = "_init_heap"]
    pub fn init_heap();

    // 68.12
    #[link_name = "_alloc"]
    pub fn alloc(size: usize) -> *const u8;

    // 68.13
    #[link_name = "_free"]
    pub fn free(block: *const u8) -> bool;

    // 68.19
    #[link_name = "_load_dll"]
    pub fn load_dll(name: *const u8) -> *const u32;
}
