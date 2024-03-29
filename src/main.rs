mod platform;

use log::debug;
use platform::{
    run_event_listener, send_backspace, send_string, KEY_DELETE, KEY_ENTER, KEY_ESCAPE, KEY_SPACE,
    KEY_TAB,
};
use std::{cmp::Ordering, sync::Mutex};

static mut TYPING_BUF: Mutex<Vec<char>> = Mutex::new(vec![]);

fn event_handler(keycode: char, shift: bool) -> bool {
    unsafe {
        let mut typing_buf = TYPING_BUF.lock().unwrap();
        match keycode {
            KEY_ENTER | KEY_TAB | KEY_SPACE | KEY_ESCAPE => {
                typing_buf.clear();
            }
            KEY_DELETE => {
                typing_buf.pop();
            }
            c => {
                typing_buf.push(if shift { c.to_ascii_uppercase() } else { c });
            }
        }

        // TELEX for now, checking if the last key is where the vietnamese tone happen
        if ['a', 'e', 'o', 'd', 's', 't', 'j', 'f', 'x', 'r', 'w'].contains(&keycode) {
            let ret = vi::telex::transform_buffer(typing_buf.as_slice());
            if ret.chars().cmp(typing_buf.clone().into_iter()) != Ordering::Equal {
                debug!("BUF {:?} - RET {:?}", typing_buf, ret);
                let backspace_count = typing_buf.len();
                debug!("  DEL {} - SEND {}", backspace_count, ret);
                _ = send_backspace(backspace_count);
                _ = send_string(&ret);
                *typing_buf = ret.chars().collect();
                return true;
            }
        }
    }
    false
}

fn main() {
    env_logger::init();
    run_event_listener(&event_handler);
}
