use alloc::vec::Vec;
use hashbrown::HashMap;

use crate::videopoker::{card::{Card, CardValue::{self, *}}, hand::Hand::*};


#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Hand {
    HighCard = 0,
    JacksOrHigher = 1,
    TwoPair = 2,
    ThreeOfAKind = 3,
    Straight = 4,
    Flush = 5,
    FullHouse = 6,
    FourOfAKind = 7,
    StraightFlush = 8,
    RoyalFlush = 9,
}

/// Takes a hand of 5 cards and return the highest achieved poker hand.
pub fn analyse_hand(hand: &Vec<Card>) -> Hand{
    let mut result = HighCard;

    let straight = is_straight(hand); // save result as it will be used again later

    if is_flush(hand) {
        if straight {
            if highest_card_value(hand) == HighAce {
                result = set_better_hand(result, RoyalFlush);
                return result;
            }
            result = set_better_hand(result, StraightFlush);
            return result;
        }
        result = set_better_hand(result, Flush);
    }

    if straight {
        result = set_better_hand(result, Straight);
    }

    match number_of_same_cards(hand) {
        5 => {//must be fullhouse (no cheating)
            result = set_better_hand(result, FullHouse);
        }
        4 => {//foak or two pair 
            let mut check_hand = get_only_multiples(hand);
            check_hand.sort_by_key(|card| card.get_value());

            if check_hand[0].get_value() == check_hand[2].get_value() {
                result = set_better_hand(result, FourOfAKind);
            }
            else {
                result = set_better_hand(result, TwoPair);
            }
        }
        3 => {//must be toak
            result = set_better_hand(result, ThreeOfAKind);
        }
        2 => {//check highest value, else highcard
            if highest_card_value(&get_only_multiples(hand)) >= Jack {
                result = set_better_hand(result, JacksOrHigher);
            }
            else {
                result = set_better_hand(result, HighCard);
            }
        }
        _ => {
            result = set_better_hand(result, HighCard);//not necessarry
        }
    }
    result
}

/// Returns the total number of all cards that are present more than once (e.g. 3 Kings and 2 Queens -> 5).
fn number_of_same_cards(hand: &Vec<Card>) -> usize {
    let analysed_hand = get_only_multiples(hand);
    return analysed_hand.len();
}

/// Removes all cards that are only present once in a given hand.
fn get_only_multiples(hand: &Vec<Card>) -> Vec<Card> {
    let mut counter_map = HashMap::new();
    for card in hand {
        *counter_map.entry(card.get_value_num()).or_insert(0) += 1;
    }

    let analysed_hand: Vec<Card> = hand
    .iter()
    .filter(|card| *counter_map.get(&card.get_value_num()).unwrap() > 1)
    .cloned()
    .collect();
    //println!("Map: {:?}", counter_map);    
    //println!("Filtered Hand ({}): {:?}", analysed_hand.len(), analysed_hand);
    return analysed_hand;
}

/// Returns the highest cards value present in the hand.
fn highest_card_value(hand: &Vec<Card>) -> CardValue {
    let mut analysed_hand: Vec<Card> = hand.clone();
    analysed_hand.sort_by_key(|card| card.get_value());
    //println!("{:?} {:?}", analysed_hand[analysed_hand.len()-1].get_value(), analysed_hand);

    if analysed_hand[0].get_value() == LowAce {
        return HighAce;
    }
    else {
        return analysed_hand[analysed_hand.len()-1].get_value();
    }
}

/// Checks if hand is a flush.
pub fn is_flush(hand: &Vec<Card>) -> bool {
    let compare_color = hand[0].get_color();
    for card in hand {
        if card.get_color() != compare_color {
            return false;
        }
    }
    true
}

/// Checks if hand is a straight, with a case distinction for aces.
pub fn is_straight(hand: &Vec<Card>) -> bool {
    let mut analysed_hand: Vec<Card> = hand.clone();
    analysed_hand.sort_by_key(|card| card.get_value());
    //println!("SA {:?}", analysed_hand);
    for i in 0..4 {
        if (analysed_hand[i].get_value_num() + 1) != analysed_hand[i+1].get_value_num() {
            //return false;
            if analysed_hand[0].get_value_num() == 1 {
                analysed_hand[0].set_ace_to_high();
                analysed_hand.sort_by_key(|card| card.get_value());
                //println!("SA2 {:?}", analysed_hand);
                for j in 0..4 {//Check again with one high ace (not recursive or else it might mess up ordering and deliver wrong results)
                    if (analysed_hand[j].get_value_num() + 1) != analysed_hand[j+1].get_value_num() {
                        return false;
                    }
                }
            }
            else {
                return false;
            }
        }
    }
    true
}

/// Returns the better hand value of two given hands.
pub fn set_better_hand(a: Hand, b: Hand) -> Hand{
    if a as usize > b as usize {
        a
    }
    else {
        b
    }
}