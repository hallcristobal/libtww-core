use addrs::game::EVENT_CANCEL;
use system::memory;

pub fn event_cancel() -> bool {
    memory::read(EVENT_CANCEL)
}

pub fn set_event_cancel(b: bool) {
    memory::write(EVENT_CANCEL, b);
}
