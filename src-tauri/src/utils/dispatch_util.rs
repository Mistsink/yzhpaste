use chrono::Local;
use rdev::{simulate, EventType, Key, SimulateError};
use std::{thread, time};

pub fn request_permissions() {
    // Simply press and release the shift key. First time the OS will ask for permissions, then do it without asking.
    dispatch(&EventType::KeyPress(Key::ControlLeft));
    dispatch(&EventType::KeyRelease(Key::ControlLeft));
}

pub fn paste() {
    println!("[{}] in dispatch paste", Local::now());

    let modifier_key = if cfg!(target_os = "macos") {
        Key::MetaLeft
    } else {
        Key::ControlLeft
    };

    // Press modifier key (Command on macOS, Control on other platforms)
    dispatch(&EventType::KeyPress(modifier_key));

    // Press 'V' key
    dispatch(&EventType::KeyPress(Key::KeyV));

    // Release 'V' key
    dispatch(&EventType::KeyRelease(Key::KeyV));

    // Release modifier key (Command on macOS, Control on other platforms)
    dispatch(&EventType::KeyRelease(modifier_key));

    println!("[{}] out dispatch paste", Local::now());
}

fn dispatch(event_type: &EventType) {
    match simulate(event_type) {
        Ok(()) => (),
        Err(SimulateError) => {
            println!("We could not dispatch {:?}", event_type);
        }
    }
    // println!("[{}] in dispatch after match", Local::now());
    // Let the OS catchup (at least MacOS)
    sleep(20); // 时间不能太短！！！不然系统反应不过来！！！
}

pub fn sleep(ms: u64) {
    let delay = time::Duration::from_millis(ms);
    thread::sleep(delay);
}
