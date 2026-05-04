/*
 * Contains demos for text output and keyboard input.
 *
 * Author: Michael Schoetter, Heinrich Heine University Duesseldorf
 *         Fabian Ruhland, Heinrich Heine University Duesseldorf, 2026-01-14
 * License: GPLv3
 */

use crate::device::{key::Scancode, keyboard::Keyboard, terminal};

/// A simple text demo, displaying formatted numbers.
pub fn text_demo() {
    //todo!("lesson1::text_demo() not implemented yet");
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

    let mut keyboard: Keyboard = Keyboard::new();
    keyboard.set_repeat_rate(0, 0);

    loop {
        let key_event = keyboard.poll_key_press();
        println!("{:?}", key_event);
        //print!("{}", keyboard.poll_key_press().ascii().unwrap_or(' '));
        if key_event.scancode() == Some(Scancode::Escape) {
            break;
        }
    }
    println!("Finished :)");
}