/*
 * Contains a demo for testing system calls.
 *
 * Author: Fabian Ruhland, Heinrich Heine University Duesseldorf, 2026-05-28
 * License: GPLv3
 */

use alloc::format;
use alloc::string::ToString;

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
    let sus = usr_get_system_time();
    let id = usr_get_thread_id();
    usr_print("test");


    usr_thread_exit();
    loop {
        // TODO: Extend demo to show more system calls.
    }
}