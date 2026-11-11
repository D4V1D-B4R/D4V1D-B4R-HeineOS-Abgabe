/*
 * Contains a demo for testing system calls.
 *
 * Author: Fabian Ruhland, Heinrich Heine University Duesseldorf, 2026-05-28
 * License: GPLv3
 */

use alloc::format;
use alloc::string::String;

use crate::device::key::Scancode;
use crate::syscall::user_api::*;
use crate::thread::scheduler::scheduler;
use crate::thread::thread::Thread;

/// Start the syscall demo.
/// It reads keyboard input and prints to the terminal from a user thread.
pub fn syscall_demo() {
    let thread = Thread::new_user_thread(syscall_test_thread);
    let scheduler = scheduler();

    scheduler.ready(thread);
    scheduler.schedule();
}

/// Thread function for the syscall demo.
/// It reads a line from the keyboard, and once the user presses 'Enter',
/// the given line and the current system time are printed to the terminal.
fn syscall_test_thread() {
    usr_hello_world();
    usr_print("System Call Demo: Type something and press enter to see the output.\n");
    let id = usr_get_thread_id();
    usr_print(&format!("Thread ID: {}\n", id));

    loop {
        let mut buffer = String::new();

        loop {
            let x = usr_get_key_event();
            if x.is_some() && x.unwrap().pressed() {
                if x.unwrap().scancode().unwrap() == Scancode::Enter {
                    break;
                }
                else {
                    buffer.push(x.unwrap().ascii().unwrap_or(' '));
                }
            }
        }
        usr_print(&format!("You typed: '{}'", buffer));
        usr_print(&format!("Sytem time: {}s\n\n", usr_get_system_time() / 1000));
    }
}