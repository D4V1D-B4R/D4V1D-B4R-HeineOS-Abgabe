use core::fmt::Write;

use alloc::string::ToString;
/*
 * Contains demos for coroutines and threads.
 *
 * Author: Michael Schoetter, Heinrich Heine University Duesseldorf
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 2026-01-15
 * License: GPLv3
 */
use log::info;
use crate::coroutine::coroutine::Coroutine;
use crate::device::pit;
use crate::device::terminal::{Terminal, terminal};
use crate::thread;
use crate::thread::scheduler::{Scheduler, scheduler};
use crate::thread::thread::Thread;

/// A demo function showcasing coroutines.
/// It starts three coroutines, each incrementing a counter and printing it to the terminal in an endless loop.
/// The coroutines switch to the next coroutine after each print.
pub fn coroutine_demo() {
    //todo!("lesson4::coroutine_demo() is not implemented yet.");
    println!("Couroutine demo:");
    println!("This demo cannot be exited. Please reboot the systgem to get back to the menu.");
    let mut corout_1 = Coroutine::new(coroutine_loop);
    let mut corout_2 = Coroutine::new(coroutine_loop);
    let mut corout_3 = Coroutine::new(coroutine_loop);

    corout_1.set_next(&mut corout_2);
    corout_2.set_next(&mut corout_3);
    corout_3.set_next(&mut corout_1);

    corout_1.start();
}

/// The function executed by each coroutine in the coroutine demo.
/// It increments a counter and prints it to the terminal in an endless loop,
/// switching to the next coroutine after each print.
fn coroutine_loop(coroutine: &mut Coroutine) {
    let mut counter = 0;
    loop {
        terminal().lock().set_pos(25, 30 + coroutine.id() * 8);
        print!("Coroutine [{}]: {}", coroutine.id(), counter);
        counter += 7;
        coroutine.switch();
    }
}

/// A demo function showcasing threads.
/// It starts three threads, each incrementing a counter and printing it to the terminal in an endless loop.
/// The threads yield the CPU to the next thread after each print.
/// The first thread also kills the other two threads after a certain number of iterations and finally exits itself, ending the demo.
pub fn thread_demo() {
    println!("Thread demo:");
    println!("This demo cannot be exited. Please reboot the systgem to get back to the menu.");
    let shed = thread::scheduler::scheduler();
    //todo!("lesson4::thread_demo() is not implemented yet.");
    let thread_1 = Thread::new(thread_entry);
    let thread_2 = Thread::new(thread_entry);
    let thread_3 = Thread::new(thread_entry);

    shed.ready(thread_1);
    shed.ready(thread_2);
    shed.ready(thread_3);
    println!("{}", shed);
    shed.schedule();
}

/// The function executed by each thread in the thread demo.
/// It increments a counter and prints it to the terminal in an endless loop,
/// yielding the CPU to the next thread after each print.
fn thread_entry() {
    let mut counter = 0;
    let start_time = pit::system_time();
    let id = scheduler().get_active_tid();
    loop {
        terminal().lock().set_pos(25, 30 + id * 8);
        print!("Thread [{}]: {}", id, counter);
        counter += 77*id;
        pit::wait(100);
        /*if counter % 10 == 0{
            scheduler().yield_cpu();
        }*/
        if counter > 10000 {
            break;
        }
    }

    terminal().lock().set_pos(25, 30 + id * 8);
    print!("Thread [{}]: {} (Finished after {} ms)", id, counter, pit::system_time() - start_time);
}