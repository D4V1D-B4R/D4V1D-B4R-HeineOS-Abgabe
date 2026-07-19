use alloc::vec::{self, Vec};

use crate::videopoker::card::{Card, CardColor, CardValue};

#[cfg(test)]
mod tests {
    use alloc::vec;
    use crate::videopoker::{card::CardValue::*, hand::{self, Hand::*}, test::{generate_flush_of_values, generate_hand_of_values}};

    #[test]
    fn test_is_royal_flush() {
        let hand = generate_flush_of_values(vec![LowAce, King, Queen, Jack, Ten]);
        assert_eq!(hand::analyse_hand(&hand), RoyalFlush);
    }

    #[test]
    fn test_is_straight_flush() {
        let hand = generate_flush_of_values(vec![King, Queen, Jack, Ten, Nine]);
        assert_eq!(hand::analyse_hand(&hand), StraightFlush);
    }

    #[test]
    fn test_is_four_of_a_kind() {
        let hand = generate_hand_of_values(vec![LowAce, LowAce, LowAce, LowAce, King]);
        assert_eq!(hand::analyse_hand(&hand), FourOfAKind);
    }

    #[test]
    fn test_is_full_house() {
        let hand = generate_hand_of_values(vec![LowAce, LowAce, LowAce, Jack, Jack]);
        assert_eq!(hand::analyse_hand(&hand), FullHouse);
    }

    #[test]
    fn test_is_flush() {
        let hand = generate_flush_of_values(vec![LowAce, Queen, Ten, Eight, Six]);
        assert_eq!(hand::analyse_hand(&hand), Flush);
    }

    #[test]
    fn test_is_straight() {
        let hand = generate_hand_of_values(vec![LowAce, King, Queen, Jack, Ten]);
        assert_eq!(hand::analyse_hand(&hand), Straight);
    }

    #[test]
    fn test_is_three_of_a_kind() {
        let hand = generate_hand_of_values(vec![LowAce, LowAce, LowAce, King, Queen]);
        assert_eq!(hand::analyse_hand(&hand), ThreeOfAKind);
    }

    #[test]
    fn test_is_two_pair() {
        let hand = generate_hand_of_values(vec![LowAce, LowAce, King, King, Queen]);
        assert_eq!(hand::analyse_hand(&hand), TwoPair);
    }

    #[test]
    fn test_is_jacks_or_higer() {
        let hand = generate_hand_of_values(vec![Jack, Jack, Ten, Nine, Eight]);
        assert_eq!(hand::analyse_hand(&hand), JacksOrHigher);
    }

    #[test]
    fn test_is_high_card() {
        let hand = generate_hand_of_values(vec![LowAce, Queen, Ten, Eight, Six]);
        assert_eq!(hand::analyse_hand(&hand), HighCard);
    }

    // Reverse Order testing

    #[test]
    fn test_is_royal_flush_reverse() {
        let mut hand = generate_flush_of_values(vec![LowAce, King, Queen, Jack, Ten]);
        hand.reverse();
        assert_eq!(hand::analyse_hand(&hand), RoyalFlush);
    }

    #[test]
    fn test_is_straight_flush_reverse() {
        let mut hand = generate_flush_of_values(vec![King, Queen, Jack, Ten, Nine]);
        hand.reverse();
        assert_eq!(hand::analyse_hand(&hand), StraightFlush);
    }

    #[test]
    fn test_is_four_of_a_kind_reverse() {
        let mut hand = generate_hand_of_values(vec![LowAce, LowAce, LowAce, LowAce, King]);
        hand.reverse();
        assert_eq!(hand::analyse_hand(&hand), FourOfAKind);
    }

    #[test]
    fn test_is_full_house_reverse() {
        let mut hand = generate_hand_of_values(vec![LowAce, LowAce, LowAce, Jack, Jack]);
        hand.reverse();
        assert_eq!(hand::analyse_hand(&hand), FullHouse);
    }

    #[test]
    fn test_is_flush_reverse() {
        let mut hand = generate_flush_of_values(vec![LowAce, Queen, Ten, Eight, Six]);
        hand.reverse();
        assert_eq!(hand::analyse_hand(&hand), Flush);
    }

    #[test]
    fn test_is_straight_reverse() {
        let mut hand = generate_hand_of_values(vec![LowAce, King, Queen, Jack, Ten]);
        hand.reverse();
        assert_eq!(hand::analyse_hand(&hand), Straight);
    }

    #[test]
    fn test_is_three_of_a_kind_reverse() {
        let mut hand = generate_hand_of_values(vec![LowAce, LowAce, LowAce, King, Queen]);
        hand.reverse();
        assert_eq!(hand::analyse_hand(&hand), ThreeOfAKind);
    }

    #[test]
    fn test_is_two_pair_reverse() {
        let mut hand = generate_hand_of_values(vec![LowAce, LowAce, King, King, Queen]);
        hand.reverse();
        assert_eq!(hand::analyse_hand(&hand), TwoPair);
    }

    #[test]
    fn test_is_jacks_or_higer_reverse() {
        let mut hand = generate_hand_of_values(vec![Jack, Jack, Ten, Nine, Eight]);
        hand.reverse();
        assert_eq!(hand::analyse_hand(&hand), JacksOrHigher);
    }

    #[test]
    fn test_is_high_card_reverse() {
        let mut hand = generate_hand_of_values(vec![LowAce, Queen, Ten, Eight, Six]);
        hand.reverse();
        assert_eq!(hand::analyse_hand(&hand), HighCard);
    }

    //Edge case testing

    #[test]
    fn test_pair_of_tens_is_not_jacks_or_higer() {
        let hand = generate_hand_of_values(vec![Ten, Queen, Ten, Eight, Six]);
        assert_eq!(hand::analyse_hand(&hand), HighCard);
    }

    #[test]
    fn test_pair_of_aces_is_jacks_or_higer() {
        let hand = generate_hand_of_values(vec![LowAce, LowAce, Ten, Eight, Six]);
        assert_eq!(hand::analyse_hand(&hand), JacksOrHigher);
    }

    #[test]
    fn test_flush_four_is_four_of_a_kind() {
        let hand = generate_flush_of_values(vec![LowAce, LowAce, LowAce, LowAce, Six]);
        assert_eq!(hand::analyse_hand(&hand), FourOfAKind);
    }

    #[test]
    fn test_straight_with_low_ace() {
        let hand = generate_hand_of_values(vec![LowAce, Two, Three, Four, Five]);
        assert_eq!(hand::analyse_hand(&hand), Straight);
    } 
}

#[allow(dead_code)]
/// Generates a Hand of given values that is guaranteed not to be a flush.
fn generate_hand_of_values(value_vec: Vec<CardValue>) -> Vec<Card>{
    let mut ret_hand: Vec<Card> = Vec::new();
    let colors: Vec<usize> = Vec::from([1,2,3,4,1]);
    for i in 0..5 {
        ret_hand.push(Card::new(CardColor::try_from(colors[i]).unwrap(), value_vec[i]));
    }
    return ret_hand;
}

#[allow(dead_code)]
/// Generates a Hand of given values that is guaranteed to be a at least a flush.
fn generate_flush_of_values(value_vec: Vec<CardValue>) -> Vec<Card>{
    let mut ret_hand: Vec<Card> = Vec::new();
    for i in 0..5 {
        ret_hand.push(Card::new(CardColor::try_from(1).unwrap(), value_vec[i]));
    }
    return ret_hand;
}