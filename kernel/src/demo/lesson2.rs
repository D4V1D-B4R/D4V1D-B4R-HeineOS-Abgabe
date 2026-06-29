/*
 * Contains a demo for heap allocations.
 *
 * Author: Michael Schoetter, Heinrich Heine University Duesseldorf
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 2026-01-14
 * License: GPLv3
 */

use alloc::boxed::Box;
use alloc::vec::Vec;
use crate::allocator;
use crate::device::key::Scancode;
use crate::device::keyboard::keyboard_buffer;
use crate::device::speaker::{self, *};

/// A simple heap demo, allocating and freeing memory on the heap.
/// The allocator state is dumped before and after each operation.
pub fn heap_demo() {
    allocator::global::dump_free_list();
    {
        println!("Allocate Box::new(1)");

        let mut vec = Vec::new();
        for _ in 0..2 {
            vec.push(Box::new(1));
        }
        println!("Done");
        allocator::global::dump_free_list();

        let a = Box::new(1);
        allocator::global::dump_free_list();
        println!("Allocate Vec::from([1,2,3]");
        let b: Vec<usize> = Vec::from([1,2,3]);
        allocator::global::dump_free_list();
        println!("{}{:?}", a, b);
    }
    println!("Drop all");
    allocator::global::dump_free_list();
    let mut vec = Vec::new();
        for _ in 0..2 {
            vec.push(Box::new(1));
        }
    allocator::global::dump_free_list();
    
}

/// A demo that plays songs via the PC speaker.
pub fn speaker_demo() {
    //speaker::sisters();
    //speaker::ddr();
    speaker::tetris();
    speaker::aerodynamic();
}

pub fn piano_demo() {
    println!("Piano Demo:");
    println!("Press keys on your keyboard. Press 'Esc' to exit the demo.\n");
    let key_buffer = keyboard_buffer();

    loop {
        let key_event = key_buffer.pop_key_event();
        let speaker = &SPEAKER;
        println!("{:?}", key_buffer.poll_key_event());
        if key_event.is_some() {
            match key_event.unwrap().scancode() {
                Some(Scancode::A) => {
                    speaker.lock().play(C1, 10);
                }
                Some(Scancode::S) => {
                    speaker.lock().play(D1, 10);
                }
                Some(Scancode::D) => {
                    speaker.lock().play(E1, 10);
                }
                Some(Scancode::F) => {
                    speaker.lock().play(F1, 10);
                }
                Some(Scancode::G) => {
                    speaker.lock().play(G1, 10);
                }
                Some(Scancode::H) => {
                    speaker.lock().play(A1, 10);
                }
                Some(Scancode::J) => {
                    speaker.lock().play(B1, 10);
                }
                Some(Scancode::Escape) => {
                    speaker.lock().off();
                    return;
                }
                _ => {
                    speaker.lock().off();
                }
            }
        }
        else {
            speaker.lock().off();
        }
    }
    println!("Finished :)");
}