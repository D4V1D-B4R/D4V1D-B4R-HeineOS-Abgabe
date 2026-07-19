use core::fmt;

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CardColor {
    Heart,
    Diamond,
    Spade,
    Club,
}

impl TryFrom<usize> for CardColor {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CardColor::Heart),
            2 => Ok(CardColor::Diamond),
            3 => Ok(CardColor::Spade),
            4 => Ok(CardColor::Club),
            _ => Err(()),
        }
    }
}

#[repr(usize)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum CardValue {
    LowAce = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5,
    Six = 6,
    Seven = 7,
    Eight = 8,
    Nine = 9,
    Ten = 10,
    Jack = 11,
    Queen = 12,
    King = 13,
    HighAce = 14,
}

impl TryFrom<usize> for CardValue {
    type Error = ();

    fn try_from(value: usize) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(CardValue::LowAce),
            2 => Ok(CardValue::Two),
            3 => Ok(CardValue::Three),
            4 => Ok(CardValue::Four),
            5 => Ok(CardValue::Five),
            6 => Ok(CardValue::Six),
            7 => Ok(CardValue::Seven),
            8 => Ok(CardValue::Eight),
            9 => Ok(CardValue::Nine),
            10 => Ok(CardValue::Ten),
            11 => Ok(CardValue::Jack),
            12 => Ok(CardValue::Queen),
            13 => Ok(CardValue::King),
            14 => Ok(CardValue::HighAce),
            _ => Err(()),
        }
    }
}

#[derive(Clone, Debug)]
pub struct Card {
    color: CardColor,
    value: CardValue,
}

impl Card {
    pub fn new(color: CardColor, value: CardValue) -> Self {
        Self { color, value }
    }

    pub fn new_from_nums(color: usize, value: usize) -> Card {
        let col = CardColor::try_from(color).unwrap();
        let val = CardValue::try_from(value).unwrap();

        Card::new(col, val)
    }

    pub fn get_color(&self) -> CardColor {
        return self.color;
    }

    pub fn get_value(&self) -> CardValue {
        return self.value;
    }

    #[allow(dead_code)]
    pub fn get_color_num(&self) -> usize {
        return self.color as usize;
    }

    pub fn get_value_num(&self) -> usize {
        return self.value as usize;
    }

    pub fn set_ace_to_high(&mut self) {
        self.value = CardValue::HighAce;
    }
}

impl fmt::Display for Card {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.value == CardValue::HighAce || self.value == CardValue::LowAce {
            write!(f, "[Ace of {:?}s]", self.color)
        }
        else {
            write!(f, "[{:?} of {:?}s]", self.value, self.color)
        }
    }
}