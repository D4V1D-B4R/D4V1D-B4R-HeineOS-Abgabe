/*
 * Contains a demo for heap allocations.
 *
 * Author: Michael Schoetter, Heinrich Heine University Duesseldorf
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 2026-01-14
 * License: GPLv3
 */

use alloc::boxed::Box;
use alloc::vec::Vec;
use log::info;
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
    let key_buffer = keyboard_buffer();
    let mut is_played = false;
    let speaker = &SPEAKER;

    println!("Piano Demo:");
    println!("Press keys on your keyboard. Press 'Esc' to exit the demo. Below you see the keyboard layout:\n");
    println!("      W E   T Z U   O P");
    println!("     A S D F G H J K L\n");

    loop {
        let key_event = key_buffer.poll_key_event();
        if key_event.pressed() && !is_played{
            match key_event.scancode() {
                Some(Scancode::A) => {
                    speaker.lock().play_long(C1);
                    is_played = true;
                }
                Some(Scancode::W) => {
                    speaker.lock().play_long(C1X);
                    is_played = true;
                }
                Some(Scancode::S) => {
                    speaker.lock().play_long(D1);
                    is_played = true;
                }
                Some(Scancode::E) => {
                    speaker.lock().play_long(D1X);
                    is_played = true;
                }
                Some(Scancode::D) => {
                    speaker.lock().play_long(E1);
                    is_played = true;
                }
                Some(Scancode::F) => {
                    speaker.lock().play_long(F1);
                    is_played = true;
                }
                Some(Scancode::T) => {
                    speaker.lock().play_long(F1X);
                    is_played = true;
                }
                Some(Scancode::G) => {
                    speaker.lock().play_long(G1);
                    is_played = true;
                }
                 Some(Scancode::Z) => {
                    speaker.lock().play_long(G1X);
                    is_played = true;
                }
                Some(Scancode::H) => {
                    speaker.lock().play_long(A1);
                    is_played = true;
                }
                Some(Scancode::U) => {
                    speaker.lock().play_long(A1X);
                    is_played = true;
                }
                Some(Scancode::J) => {
                    speaker.lock().play_long(B1);
                    is_played = true;
                }
                Some(Scancode::K) => {
                    speaker.lock().play_long(C2);
                    is_played = true;
                }
                Some(Scancode::O) => {
                    speaker.lock().play_long(C2X);
                    is_played = true;
                }
                Some(Scancode::L) => {
                    speaker.lock().play_long(D2);
                    is_played = true;
                }
                Some(Scancode::P) => {
                    speaker.lock().play_long(D2X);
                    is_played = true;
                }
                Some(Scancode::Escape) => {
                    speaker.lock().off();
                    break;
                }
                _ => {
                    speaker.lock().off();
                    is_played = false;
                }
            }
        }
        else if !key_event.pressed(){
            speaker.lock().off();
            is_played = false;
        }
    }
    println!("Finished :)");
}