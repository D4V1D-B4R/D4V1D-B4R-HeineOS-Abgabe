use crate::device::{key::KeyEvent, keyboard::keyboard_buffer};

pub extern "C" fn sys_get_key_event(key_event_ptr: *mut KeyEvent) -> u64 {
    let key_buffer = keyboard_buffer();
    let key_event = key_buffer.poll_key_event();
    if key_event.ascii().is_some() {
        unsafe { *key_event_ptr = key_event };
        return 1;
    }
    return 0;
}