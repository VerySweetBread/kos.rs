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

    // 10
    #[link_name = "_wait_event"]
    pub fn wait_event() -> u32;

    // 12.1
    #[link_name = "_start_window_draw"]
    pub fn start_window_draw();

    // 12.2
    #[link_name = "_end_window_draw"]
    pub fn end_window_draw();
}
