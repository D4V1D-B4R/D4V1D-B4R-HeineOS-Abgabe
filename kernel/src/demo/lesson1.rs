/*
 * Contains demos for text output and keyboard input.
 *
 * Author: Michael Schoetter, Heinrich Heine University Duesseldorf
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 2026-01-14
 * License: GPLv3
 */

use crate::device::{key::Scancode, keyboard::keyboard_buffer};

/// A simple text demo, displaying formatted numbers.
pub fn text_demo() {
    println!("Text Demo:\n");
    println!("  | dec | hex | bin   |");
    println!("  |_____|_____|_______|");
    for i in 0..17 {
        println!("  |  {:2} |  {:2x} | {:5b} |", i, i, i);
    }
}

/// A simple keyboard demo, displaying the events of key presses and releases.
pub fn keyboard_demo() {
    println!("Keyboard Demo:");
    println!("Press keys on your keyboard. Press 'Esc' to exit the demo.\n");
    let key_buffer = keyboard_buffer();

    loop {
        let key_event = key_buffer.poll_key_event();
        println!("{:?}", key_event);
        if key_event.scancode() == Some(Scancode::Escape) {
            break;
        }
    }
    println!("Finished :)");
}

/*pub fn keyboard_demo() {
    println!("Keyboard Demo:");
    println!("Press keys on your keyboard. Press 'Esc' to exit the demo.\n");
    let mut keyboard: Keyboard = Keyboard::new();

    loop {
        let key_event = keyboard.poll_key_press();
        println!("{:?}", key_event);
        if key_event.scancode() == Some(Scancode::Escape) {
            break;
        }
    }
    println!("Finished :)");
}*/