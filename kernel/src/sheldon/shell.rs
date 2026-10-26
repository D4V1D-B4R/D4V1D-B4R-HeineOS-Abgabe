use core::cmp;
use alloc::{string::String, vec::Vec};

use crate::{demo, device::{framebuffer, key::Scancode, keyboard::keyboard_buffer, pit, speaker::SPEAKER, terminal::{framebuffer, terminal}}, library::random, videopoker};

// Length of the beginnig of the line which shall not be overwritten by the user
const PADDING: usize = framebuffer::CHAR_WIDTH * 3;

/// A very simple shell to start other programms from
pub fn shell() {
    println!("Welcome to HeineOS(C). Please type in your command (or \"help\" for... well, help.):");
    print!(">> ");
    let key_buffer = keyboard_buffer();
    
    let mut my_buff: Vec<char> = Vec::new();
    let mut previous: Vec<char> = Vec::new();

    // Main input loop
    loop {
        let key_event = key_buffer.poll_key_event();

        if key_event.pressed() {
            match key_event.scancode() {

                // Confirm input and process it
                Some(Scancode::Enter) => {
                    println!("");
                    let start_time = pit::system_time();
                    let print_end = process_query(&my_buff);
                    let finish_time = pit::system_time();
                    if print_end {
                        println!("\n>> Task finished in {} ms.", finish_time - start_time);
                    }
                    print!(">> ");

                    // Set previous buffer
                    previous = my_buff.clone();
                    my_buff.clear();
                },

                // Delete typed in characters (from screen & buffer), without deleting the beginning of the line
                Some(Scancode::Backspace) => {
                    my_buff.pop();

                    let cursor_pos: (usize, usize) = terminal().lock().pos();
                    let col = cmp::max(cursor_pos.0
                        .checked_sub(framebuffer::CHAR_WIDTH).unwrap_or(0), PADDING);
                    terminal().lock().set_pos(col, cursor_pos.1);
                },

                // Brings up the last typed in command
                Some(Scancode::Up) => {
                    SPEAKER.lock().play_blip();
                    my_buff = previous.clone();
                    let printable: String = my_buff.iter().collect();
                    let row = terminal().lock().pos().1;
                    framebuffer().lock().clear_line(row);
                    terminal().lock().set_pos(0, row);
                    print!(">> {}", printable);
                }

                // Clears the line and input
                Some(Scancode::Down) => {
                    my_buff.clear();
                    let row = terminal().lock().pos().1;
                    framebuffer().lock().clear_line(row);
                    terminal().lock().set_pos(0, row);
                    print!(">> ");
                }

                // Clears screen and quits the shell
                Some(Scancode::Escape) => {
                    terminal().lock().clear();
                    break;
                }

                // Print the given char and add it to the buffer
                _ => {
                    let c = key_event.ascii().unwrap_or(' ');
                    my_buff.push(c);

                    print!("{}", c);
                },
            }
        }
    }

    //Exit shell
    SPEAKER.lock().play_reject();
    println!("Exiting shell...");
}

/// Processes the query and performs the correspondin action.
/// Returned bool tells wheter to print runtime after finishing or not.
fn process_query(buffer: &Vec<char>) -> bool {
    let command: String = buffer.iter().collect();

    match command.as_str() {
        // Given code
        "text_demo" => { demo::lesson1::text_demo(); },
        "keyboard_demo" => { demo::lesson1::keyboard_demo(); },
        "heap_demo" => { demo::lesson2::heap_demo(); },
        "speaker_demo" => { demo::lesson2::speaker_demo(); },
        "coroutine_demo" => { demo::lesson4::coroutine_demo(); },
        "thread_demo" => { demo::lesson4::thread_demo(); },
        "peanut_gb" => { demo::lesson6::peanut_gb::play("/roms/2048.gb");},
        "pci_print_devices_demo" => { demo::lesson7::print_pci_devices(); },
        "pci_rtl_demo" => { demo::lesson7::rtl8139_demo(); },
        "lesson8" => {demo::lesson8::user_thread_demo();}

        // My stuff
        "piano_demo" => { demo::lesson2::piano_demo(); },
        "random_with_limit_demo" => { demo::random_demo::random_with_limit_demo(5, random::DEFAULT_SEED_1, 10000, 5); },
        "random_with_range_demo" => { demo::random_demo::random_with_range_demo(5, 10, random::DEFAULT_SEED_1, 10000, 5); },
        "videopoker" => { videopoker::poker::play(); },
        "typing_demo" => { demo::typing_demo::typing_demo(); },

        // Basic shell stuff
        "clear" => { terminal().lock().clear(); return false; },
        "heinrich" => { print!("heine") },
        "help" => { help(); },
        "" => { return false; },

        // Unknown command
        _ => {  SPEAKER.lock().play_reject();
                print!("Unknown command \"{}\". Please try again or type \"help\" to get help.\n", command.as_str()); return false;
        },
    }
    true
}

/// Prints help (all aviable commands)
fn help() {
    let mut commands = Vec::from([
        "text_demo",
        "keyboard_demo",
        "heap_demo",
        "speaker_demo",
        "coroutine_demo",
        "thread_demo",
        "peanut_gb",
        "random_with_limit_demo",
        "random_with_range_demo",
        "videopoker",
        "typing_demo",
        "clear",
        "heinrich",
        "help",
        "pci_print_devices_demo",
        "pci_rtl_demo" ,
        "lesson8"
    ]);

    commands.sort();

    println!("The following commands are available:\n");
    for item in commands {
        println!("{}", item);
    }
    println!("\nPress Escape to close this shell. (WARNING: You will not be able to start it anymore!)");
}