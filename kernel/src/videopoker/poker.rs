use core::fmt;

use alloc::vec::Vec;
use log::info;

use crate::{device::{framebuffer, key::Scancode::{self, M}, keyboard::keyboard_buffer, pit, speaker::SPEAKER, terminal::{self, terminal}}, library::{bitmap::Bitmap, random::{self, Random}}, videopoker::{card::{Card, CardColor::{self, *}, CardValue::*}, hand}};

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

/// Generates a Vector of Bitmaps for Card dispay (Diamond, Heart, Leaf, Spade; from Ace to King).
fn init_bitmaps() -> Vec<Bitmap> {
    let mut bitmaps: Vec<Bitmap> = Vec::new();
    bitmaps.push(Bitmap::read_from_file("acediamond.bmp").unwrap().unwrap());//0
    bitmaps.push(Bitmap::read_from_file("aceheart.bmp").unwrap().unwrap());//1
    bitmaps.push(Bitmap::read_from_file("aceleaf.bmp").unwrap().unwrap());//2
    bitmaps.push(Bitmap::read_from_file("acespade.bmp").unwrap().unwrap());//3

    bitmaps.push(Bitmap::read_from_file("twodiamond.bmp").unwrap().unwrap());//4
    bitmaps.push(Bitmap::read_from_file("twoheart.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("twoleaf.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("twospade.bmp").unwrap().unwrap());

    bitmaps.push(Bitmap::read_from_file("threediamond.bmp").unwrap().unwrap());//8
    bitmaps.push(Bitmap::read_from_file("threeheart.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("threeleaf.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("threespade.bmp").unwrap().unwrap());

    bitmaps.push(Bitmap::read_from_file("fourdiamond.bmp").unwrap().unwrap());//12
    bitmaps.push(Bitmap::read_from_file("fourheart.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("fourleaf.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("fourspade.bmp").unwrap().unwrap());

    bitmaps.push(Bitmap::read_from_file("fivediamond.bmp").unwrap().unwrap());//16
    bitmaps.push(Bitmap::read_from_file("fiveheart.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("fiveleaf.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("fivespade.bmp").unwrap().unwrap());

    bitmaps.push(Bitmap::read_from_file("sixdiamond.bmp").unwrap().unwrap());//20
    bitmaps.push(Bitmap::read_from_file("sixheart.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("sixleaf.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("sixspade.bmp").unwrap().unwrap());

    bitmaps.push(Bitmap::read_from_file("sevendiamond.bmp").unwrap().unwrap());//24
    bitmaps.push(Bitmap::read_from_file("sevenheart.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("sevenleaf.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("sevenspade.bmp").unwrap().unwrap());

    bitmaps.push(Bitmap::read_from_file("eightdiamond.bmp").unwrap().unwrap());//28
    bitmaps.push(Bitmap::read_from_file("eightheart.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("eightleaf.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("eightspade.bmp").unwrap().unwrap());

    bitmaps.push(Bitmap::read_from_file("ninediamond.bmp").unwrap().unwrap());//32
    bitmaps.push(Bitmap::read_from_file("nineheart.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("nineleaf.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("ninespade.bmp").unwrap().unwrap());

    bitmaps.push(Bitmap::read_from_file("tendiamond.bmp").unwrap().unwrap());//36
    bitmaps.push(Bitmap::read_from_file("tenheart.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("tenleaf.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("tenspade.bmp").unwrap().unwrap());

    bitmaps.push(Bitmap::read_from_file("jackdiamond.bmp").unwrap().unwrap());//40
    bitmaps.push(Bitmap::read_from_file("jackheart.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("jackleaf.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("jackspade.bmp").unwrap().unwrap());

    bitmaps.push(Bitmap::read_from_file("queendiamond.bmp").unwrap().unwrap());//44
    bitmaps.push(Bitmap::read_from_file("queenheart.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("queenleaf.bmp").unwrap().unwrap());
    bitmaps.push(Bitmap::read_from_file("queenspade.bmp").unwrap().unwrap());

    bitmaps.push(Bitmap::read_from_file("kingdiamond.bmp").unwrap().unwrap());//48
    bitmaps.push(Bitmap::read_from_file("kingheart.bmp").unwrap().unwrap());//49
    bitmaps.push(Bitmap::read_from_file("kingleaf.bmp").unwrap().unwrap());//50
    bitmaps.push(Bitmap::read_from_file("kingspade.bmp").unwrap().unwrap());//51

    bitmaps.push(Bitmap::read_from_file("card.bmp").unwrap().unwrap());//52, backside

    bitmaps
}

/// Draws the correct bitmap for the given card on screen.
fn draw_card_bitmap(card: &Card, bitmaps: &Vec<Bitmap>, x: usize, y: usize) {
    let add: usize;
    let base: usize;
    
    match card.get_color() {
        CardColor::Diamond => { add = 0 },
        CardColor::Heart => { add = 1 },
        CardColor::Club => { add = 2 },
        CardColor::Spade => { add = 3 },
    }

    match card.get_value_num() {
        1 => { base = 0 },
        14 => { base = 0 },

        2 => { base = 4 },
        3 => { base = 8 },
        4 => { base = 12 },
        5 => { base = 16 },
        6 => { base = 20 },
        7 => { base = 24 },
        8 => { base = 28 },
        9 => { base = 32 },
        10 => { base = 36 },

        11 => { base = 40 },
        12 => { base = 44 },
        13 => { base = 48 },

        _ => { base = 52 },// does not happen
    }

    terminal().lock().framebuffer().lock().draw_bitmap(&bitmaps[base + add], x + 10, 16);
}


/// Draws the backside of the card on the screen.
fn draw_backside_bitmap(bitmaps: &Vec<Bitmap>, x: usize, y: usize) {
    terminal().lock().framebuffer().lock().draw_bitmap(&bitmaps[52], x + 10, 16);
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

/// Play a game of video poker
pub fn play() {
    let key_buffer = keyboard_buffer(); // Keyboard buffer instance for player input

    let mut credits: i64 = 0; // the players credits

    let mut deck: Vec<Card> = generate_full_deck(); // the full deck of cards

    // A little startup thingy that shows all cards
    for card in &deck {
        println!("{}", card);
    }
    println!("Please wait, bitmaps are being loaded...");

    let bitmaps = init_bitmaps(); // all bitmaps for all cards

    let mut hand: Vec<Card> = Vec::new(); // the current hand
    let mut selector: Vec<bool> = Vec::from([false, false, false, false, false]); // the selected cards to keep in hand
    let mut selector_pos: i64 = 2; // the current selector

    let mut is_second_loop = false; // to confirm that the selection has been made and a new round begins
    
    ////////////////////////////////////////////////////////////////////
 
    for _ in 0..5 {
        hand.push(pull_random_card_from_deck(&mut deck));
    }

    // Debug stuff if a certain hand needs to be tested
    /*hand.push(Card::new(Heart, LowAce));
    hand.push(Card::new(Diamond, Two));
    hand.push(Card::new(Heart, Three));
    hand.push(Card::new(Diamond, Four));
    hand.push(Card::new(Heart, Five));*/

    // Initial title screen before first paly
    terminal().lock().clear();
    println!("One play costs 100 Credits. Press any key to begin...");
    key_buffer.poll_key_press();
    terminal().lock().clear();

    credits -= 100;

    // Print first play screen
    redraw_all(&mut hand, &bitmaps, is_second_loop, &selector, selector_pos, credits);

    // Main game loop
    loop {
        // Input loop for reading keys
        loop {
            let key_event = key_buffer.poll_key_event();
            if key_event.pressed() {
                match key_event.scancode() {
                    // Continue with next round
                    Some(Scancode::Enter) => {
                        SPEAKER.lock().play_confirm();

                        if !is_second_loop {
                            for i in 0..selector.len() {
                                if !selector[i] {
                                    hand[i] = pull_random_card_from_deck(&mut deck);
                                }
                            }
                            is_second_loop = true;

                            credits += 100 * hand::analyse_hand(&hand) as i64;

                            redraw_cards(&hand, &selector, &bitmaps);
                            redraw_info(credits, &mut hand, is_second_loop);
                
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

                            is_second_loop = false;

                            credits -= 100;

                            redraw_all(&mut hand, &bitmaps, is_second_loop, &selector, selector_pos, credits);
                            
                            break;
                        }
                    },

                    // Go left with pointer
                    Some(Scancode::Left) => {
                        if !is_second_loop {
                            SPEAKER.lock().play_blip();
                            selector_pos = selector_pos - 1;
                            if selector_pos < 0 {
                                selector_pos = 4;
                            }
                            redraw_pointer(selector_pos);
                            break;
                        }
                    },

                    // Go right with pointer
                    Some(Scancode::Right) => {
                        if !is_second_loop {
                            SPEAKER.lock().play_blip();
                            selector_pos = (selector_pos + 1) % selector.len() as i64;
                            redraw_pointer(selector_pos);
                            break;
                        }
                    },

                    // Select Card
                    Some(Scancode::Space) => {
                        if !is_second_loop {
                            let i = selector_pos as usize;
                            selector[i] = !selector[i];
                            redraw_selector(&selector);
                            break;
                        }
                    },

                    // Exit the game
                    Some(Scancode::F) => {
                        terminal().lock().clear();
                        if credits < 0 {
                            print!("Better luck next time! You lost {} Credits in total! ", credits);
                        }
                        else {
                            print!("Thanks for playing! You won {} Credits in total!", credits);
                        }
                        return;
                    },

                    // everything else does nothing
                    _ => {},
                }
            }

            terminal().lock().set_pos(800, 600);
        }
    }
}

/// Draws all newly pulled cards. First backside, then front reveal.
fn redraw_cards(hand: &Vec<Card>, selector: &Vec<bool>, bitmaps: &Vec<Bitmap>) {
    let wait_time = 200;

    for i in 0..5 {
        if !selector[i] {
            draw_backside_bitmap(bitmaps, 160 * i, 0);
            pit::wait(wait_time);
            draw_card_bitmap(&hand[i], bitmaps, 160 * i, 0);
            pit::wait(wait_time);
        }
    }
}

/// Draws the selector and current selection.
fn redraw_selector(selector: &Vec<bool>) {
    let fb = terminal::framebuffer();

    let len = 20 * framebuffer::CHAR_WIDTH;

    let mut x = 0;
    let y = 14;

    for is_held in selector {
        if *is_held {
           fb.lock().draw_str("[     >HOLD<      ]", x, y * framebuffer::CHAR_HEIGHT, framebuffer::BLACK, framebuffer::RED);
        }
        else {
             fb.lock().draw_str("[      HOLD       ]", x, y * framebuffer::CHAR_HEIGHT, framebuffer::RED, framebuffer::BLACK);
        }
        x += len;
    }
}

/// Draws the pointer to the selector, consisitng of a simple 'A'.
fn redraw_pointer(selector_pos: i64) {
    let fb = terminal::framebuffer();

    let len = 20 * framebuffer::CHAR_WIDTH;

    let mut x = 0;
    let y = 15;

    for i in 0..5 {
        if i == selector_pos {
            fb.lock().draw_str("        A          ", x, y * framebuffer::CHAR_HEIGHT, framebuffer::RED, framebuffer::BLACK);
        }
        else {
            fb.lock().draw_str("                   ", x, y * framebuffer::CHAR_HEIGHT, framebuffer::RED, framebuffer::BLACK);
        }
        x += len;
    }
}

/// Writes info on current hand, win/loss and current credits.
fn redraw_info(credits: i64, hand: &mut Vec<Card>, is_second_loop: bool) {
    terminal().lock().set_pos(0, 25 * framebuffer::CHAR_HEIGHT);

    if !is_second_loop {
        println!("Current Hand is {:?}, Value: {}. Current Credits: {}\n",
            hand::analyse_hand(hand), hand::analyse_hand(hand) as usize, credits);
            println!("                             ");//TODO: add clear line function
    }

    else {
        let win = 100 * hand::analyse_hand(hand) as usize;
        pit::wait(500);
        if win == 0 {
            SPEAKER.lock().play_reject();
            println!("Hand was only a {:?}! You lost 100 Credits! Current Credits: {}", hand::analyse_hand(hand), credits);
        }
        else {
            SPEAKER.lock().play_confirm();
            SPEAKER.lock().play_confirm();
            SPEAKER.lock().play_confirm();
            SPEAKER.lock().play_confirm();
            println!("You won {} Credits with a {:?}! Current Credits: {}", win, hand::analyse_hand(hand), credits);
        }
        println!("Press Enter to play again.");
    }
}

/// Writes controls on screen.
fn redraw_controls() {
    terminal().lock().set_pos(0, 30 * framebuffer::CHAR_HEIGHT);

    println!("Select cards to keep and try to score high! One game costs 100 Credits.");
    println!("[<-][->] Move the pointer with the arrow keys.");
    println!("[ ____ ] Select a card to keep with the space bar. It will appear as >HOLD< in the selector.");
    println!("[ENTER ] Press enter to try your luck!");
    println!("[F] Press F to exit the game anytime.");
}

/// Redraws everything using all of the above
fn redraw_all(hand: &mut Vec<Card>, bitmaps: &Vec<Bitmap>, is_second_loop: bool, selector: &Vec<bool>, selector_pos: i64, credits: i64) {
    redraw_cards(hand, selector, bitmaps);
    redraw_info(credits, hand, is_second_loop);
    redraw_selector(selector);
    redraw_pointer(selector_pos);
    redraw_controls();
}
/*
TODO:
- create clear line function!!! (brauche ich das?)
- backside of cards
- facecards
- use framebuffer only?
*/