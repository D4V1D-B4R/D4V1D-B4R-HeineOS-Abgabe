use crate::device::{framebuffer, key::Scancode, keyboard::keyboard_buffer, terminal::terminal};

pub fn typing_demo() {
    println!("Keyboard Demo:");
    println!("Press keys on your keyboard. Press 'Esc' to exit the demo.\n");
    let key_buffer = keyboard_buffer();

    loop {
        let key_event = key_buffer.poll_key_event();
        if key_event.pressed() {
            match key_event.scancode() {
                Some(Scancode::Escape) => {
                    println!("");
                    break;
                },
                Some(Scancode::Enter) => {
                    println!("");
                },
                Some(Scancode::Backspace) => {
                    let cursor_pos: (usize, usize) = terminal().lock().pos();
                    terminal().lock()
                        .set_pos(cursor_pos.0.checked_sub(framebuffer::CHAR_WIDTH).unwrap_or(0) , cursor_pos.1);
                },
                Some(Scancode::Left) => {
                    let cursor_pos: (usize, usize) = terminal().lock().pos();
                    terminal().lock()
                        .set_pos(cursor_pos.0.checked_sub(framebuffer::CHAR_WIDTH).unwrap_or(0) , cursor_pos.1);
                },
                Some(Scancode::Right) => {
                    let cursor_pos: (usize, usize) = terminal().lock().pos();
                    terminal().lock()
                        .set_pos(cursor_pos.0 + framebuffer::CHAR_WIDTH, cursor_pos.1);
                },
                Some(Scancode::Up) => {
                    let cursor_pos: (usize, usize) = terminal().lock().pos();
                    terminal().lock()
                        .set_pos(cursor_pos.0, cursor_pos.1.checked_sub(framebuffer::CHAR_HEIGHT).unwrap_or(0) );
                },
                Some(Scancode::Down) => {
                    let cursor_pos: (usize, usize) = terminal().lock().pos();
                    terminal().lock()
                        .set_pos(cursor_pos.0, cursor_pos.1 + framebuffer::CHAR_HEIGHT );
                },
                _ => {
                    print!("{}", key_event.ascii().unwrap_or(' '));
                },
            }
        }
    }
    println!("Finished :)");
}