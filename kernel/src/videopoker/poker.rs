use core::fmt;

use alloc::vec::Vec;
use log::info;

use crate::{device::{framebuffer, key::Scancode, keyboard::keyboard_buffer, pit, terminal::terminal}, library::{bitmap::Bitmap, random::{self, Random}}, videopoker::{card::{Card, CardColor::*, CardValue::*}, hand}};

/// Generates a full card deck with 52 cards.
fn generate_full_deck() -> Vec<Card> {
    let mut deck: Vec<Card> = Vec::new();
    for i in 1..5 { // iterate over colors 1 to 4 (bounds: 1..5)
        for j in 1..14 { // iterate over values 1 to 13 (bounds: 1..14)
            deck.push(Card::new_from_nums(i, j));
        }
    }
    deck
}

/// Returns a random value between a and b.
fn generate_random_value_limited(limit: usize) -> usize {
    let mut rng = Random::new(random::DEFAULT_SEED_2);
    rng.random_limited(limit)
}

/// Pulls a random card from the deck and removes it.
fn pull_random_card_from_deck(deck: &mut Vec<Card>) -> Card {
    let deck_size = deck.len() - 1;
    let card_number = generate_random_value_limited(deck_size);
    deck.remove(card_number)
}

/// Prints a vector neatly in a single row.
fn print_vec_in_row<T: fmt::Display>(vec: &[T]) {
    for item in vec {
        print!("{} ", item);//(was print!) TODO
    }
    println!("");
}

/// Displays a hand of cards as graphics on the terminal.
fn display_cards(cards: &Vec<Card>) {
    // TODO: Replace with bitmaps
    println!("//////////////////////////////////////////////////////////////////////////////////////////////////");
    print_vec_in_row(&cards);
    println!("//////////////////////////////////////////////////////////////////////////////////////////////////");
}

/// Play a game of video poker
pub fn play() {
    let key_buffer = keyboard_buffer();

    let mut credits: usize = 0; // the players credits, can be added via button press
    let mut deck: Vec<Card> = generate_full_deck(); // the full deck of cards
    let mut hand: Vec<Card> = Vec::new(); // the current hand
    let mut selector: Vec<bool> = Vec::from([false, false, false, false, false]); // the selected cards to keep in hand
    let mut selector_pos: i64 = 2; // the current selector

    let mut is_second_loop = false; // to confirm that the selection has been made and a new round begins
    let mut redraw_cards = true;
    let mut redraw_info = true;
    let mut redraw_selector = true;
    let mut redraw_pointer = true;

    let bitmap2 = Bitmap::read_from_file("card.bmp").unwrap().unwrap();
    let dist = 10;
    let ypsilon = 16;

    ////////////////////////////////////////////////////////////////////

    for card in &deck {
        println!("{}", card);
    }
    println!("Total: {} cards.", &deck.len());
    /* 
    for _ in 0..5 {
        hand.push(pull_random_card_from_deck(&mut deck));
    }
    */
    hand.push(Card::new(Heart, LowAce));
    hand.push(Card::new(Diamond, Two));
    hand.push(Card::new(Heart, Three));
    hand.push(Card::new(Diamond, Four));
    hand.push(Card::new(Heart, Five));

    terminal().lock().clear();
    //Print controls
    fredraw_controls();

    // Main game loop
    loop {
        // Display cards
        if redraw_cards {
            fredraw_cards(&hand, &mut redraw_cards, &bitmap2, dist, ypsilon);
        }

        // Print current stats
        if redraw_info {
            fredraw_info(credits, &mut hand, is_second_loop, &mut redraw_info);
        }

        // Draw selector
        if redraw_selector {
            fredraw_selector(&selector, &mut redraw_selector);
        }

        //Draw pointer
        if redraw_pointer {
            fredraw_pointer(selector_pos, &mut redraw_pointer);
        }

        terminal().lock().set_pos(800, 600);

        // Input loop for reading keys
        loop {
            let key_event = key_buffer.poll_key_event();
            //println!("{:?}", key_event);
            //pit::wait(50);
            if key_event.pressed() {
                match key_event.scancode() {
                    // Continue with next round
                    Some(Scancode::Enter) => {
                        if !is_second_loop {
                            for i in 0..selector.len() {
                                if !selector[i] {
                                    hand[i] = pull_random_card_from_deck(&mut deck);
                                }
                            }
                            redraw_cards = true;
                            redraw_info = true;

                            is_second_loop = true;
                            break;
                        }
                        else {
                            deck = generate_full_deck();
                            hand.clear();
                            for _ in 0..5 {
                                hand.push(pull_random_card_from_deck(&mut deck));
                            }

                            for i in 0..selector.len() {
                                selector[i] = false;
                            }
                            selector_pos = 2;

                            terminal().lock().clear();
                            redraw_cards = true;
                            redraw_info = true;
                            redraw_selector = true;
                            redraw_pointer = true;

                            is_second_loop = false;
                            break;
                        }
                    },
                    // Go left with pointer
                    Some(Scancode::Left) => {
                        if !is_second_loop {
                            selector_pos = selector_pos - 1;
                            if selector_pos < 0 {
                                selector_pos = 4;
                            }
                            redraw_pointer = true;
                            break;
                        }
                    },
                    // Go right with pointer
                    Some(Scancode::Right) => {
                        if !is_second_loop {
                            selector_pos = (selector_pos + 1) % selector.len() as i64;
                            redraw_pointer = true;
                            break;
                        }
                    },
                    // Select Card
                    Some(Scancode::Space) => {
                        if !is_second_loop {
                            let i = selector_pos as usize;
                            selector[i] = !selector[i];
                            redraw_selector = true;
                            break;
                        }
                    },
                    // Simulate adding a coin to the machine
                    Some(Scancode::C) => {
                        credits += 100;
                        redraw_info = true;
                        break;
                    },
                    // Exit the game
                    Some(Scancode::F) => {
                        terminal().lock().clear();
                        print!("Thanks for playing!");
                        return;
                    },
                    _ => {
                        info!("poker: ???");
                    },
                }
            }
        }
    }
}

fn fredraw_controls() {
    terminal().lock().set_pos(0, 26 * framebuffer::CHAR_HEIGHT);
    println!("[C] Press C to add 100 Credits to play. One game is 100 Credits.");
    println!("[<-][->] Move the pointer with the arrow keys.");
    println!("[ ____ ] Select a card to keep with the space bar. It will appear as >HOLD< in the selector.");
    println!("[ENTER ] Press enter to try your luck!");
    println!("[F] Press F to exit the game anytime.");
}

fn fredraw_pointer(selector_pos: i64, redraw_pointer: &mut bool) {
    terminal().lock().set_pos(0, 24 * framebuffer::CHAR_HEIGHT);
    for i in 0..5 {
        if i == selector_pos {
            print!("        A           ");
        }
        else {
            print!("                    ");
        }
    }
    *redraw_pointer = false;
}

fn fredraw_selector(selector: &Vec<bool>, redraw_selector: &mut bool) {
    terminal().lock().set_pos(0, 22 * framebuffer::CHAR_HEIGHT);
    for is_held in selector {
        if *is_held {
            print!("[     >HOLD<      ] ");
        }
        else {
            print!("[      HOLD       ] ");
        }
    
    }
    *redraw_selector = false;
}

fn fredraw_info(credits: usize, hand: &mut Vec<Card>, is_second_loop: bool, redraw_info: &mut bool) {
    terminal().lock().set_pos(0, 20 * framebuffer::CHAR_HEIGHT);
    if !is_second_loop {
        println!("Current Hand is {:?}, Value: {}. Current Credits: {}\n",
            hand::analyse_hand(hand), hand::analyse_hand(hand) as usize, credits);
            println!("                             ");//TODO: add clear line function
    }
    else {
        println!("You won {} Credits with a {:?}! Current Credits: {}            ", 0, hand::analyse_hand(hand), credits);
        println!("Press Enter to play again.");
    }
    *redraw_info = false;
}

fn fredraw_cards(hand: &Vec<Card>, redraw_cards: &mut bool, bitmap2: &Bitmap, dist: usize, ypsilon: usize) {
    terminal().lock().framebuffer().lock().draw_bitmap(bitmap2, 0 + dist, ypsilon);
    terminal().lock().framebuffer().lock().draw_bitmap(bitmap2, 160 + dist, ypsilon);
    terminal().lock().framebuffer().lock().draw_bitmap(bitmap2, 320 + dist, ypsilon);
    terminal().lock().framebuffer().lock().draw_bitmap(bitmap2, 480 + dist, ypsilon);
    terminal().lock().framebuffer().lock().draw_bitmap(bitmap2, 640 + dist, ypsilon);
    terminal().lock().set_pos(0, 15 * framebuffer::CHAR_HEIGHT);
    display_cards(hand);
    // TODO: replace with function to draw cards
    *redraw_cards = false;
}


/*
TODO:
- create clear line function!!!
- use credits
- bitmaps 
*/