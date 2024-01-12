use crate::sys;

pub fn exit() -> ! {
    unsafe { sys::exit() }
}

#[non_exhaustive]
pub enum Event {
    Redraw,
    KeyPress,
}

pub fn fetch_event() -> Option<Event> {
    match unsafe { sys::wait_event() } {
        1 => Some(Event::Redraw),
        2 => Some(Event::KeyPress),
        _ => None,
    }
}
