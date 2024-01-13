use crate::sys;

pub fn exit() -> ! {
    unsafe { sys::exit() }
}

#[non_exhaustive]
pub enum Event {
    Redraw,
    KeyPress,
    BtnPress,
    BgRedraw,
    Mouse,
    IPC,
    Network,
    Debug,
}

pub fn fetch_event() -> Option<Event> {
    match unsafe { sys::wait_event() } {
        1 => Some(Event::Redraw),
        2 => Some(Event::KeyPress),
        3 => Some(Event::BtnPress),
        5 => Some(Event::BgRedraw),
        6 => Some(Event::Mouse),
        7 => Some(Event::IPC),
        8 => Some(Event::Network),
        9 => Some(Event::Debug),
        _ => None,
    }
}
