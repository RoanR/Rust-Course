
use std::fmt::Display;

use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
pub struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    pub fn new() -> Self {
        let mut cards = vec![];

        for suit in [Suits::Clubs, Suits::Diamonds, Suits::Hearts, Suits::Spades] {
            for card_number in 1..14 {
                let card = Card {
                    suit,
                    number: card_number.into(),
                };
                cards.push(card);
            }
        }

        Deck { cards }
    }

    pub fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    pub fn deal(&mut self, num_cards: usize) -> Vec<Card> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

impl Display for Deck {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "\n====================\n")?;
        for card in &self.cards {
            write!(f, "{}\n", card)?;
        }
        write!(f, "====================")
    }
}

#[derive(Debug)]
pub struct Card {
    suit: Suits,
    number: CardNumber,
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if self.number == CardNumber::Joker {
            if self.suit == Suits::Clubs || self.suit == Suits::Spades {
                write!(f, "Black {}", self.number)
            } else {
                write!(f, "Red {}", self.number)
            }
        } else {
            write!(f, "{} of {}", self.number, self.suit)
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Suits {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

impl Into<String> for Suits {
    fn into(self) -> String {
        match self {
            Suits::Clubs => "Clubs".to_string(),
            Suits::Diamonds => "Diamonds".to_string(),
            Suits::Hearts => "Hearts".to_string(),
            Suits::Spades => "Spades".to_string(),
        }
    }
}

impl Display for Suits {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s_string: String = (*self).into();
        write!(f, "{}", s_string)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CardNumber {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
    Joker,
}

impl From<usize> for CardNumber {
    fn from(value: usize) -> Self {
        match value {
            1 => CardNumber::Ace,
            2 => CardNumber::Two,
            3 => CardNumber::Three,
            4 => CardNumber::Four,
            5 => CardNumber::Five,
            6 => CardNumber::Six,
            7 => CardNumber::Seven,
            8 => CardNumber::Eight,
            9 => CardNumber::Nine,
            10 => CardNumber::Ten,
            11 => CardNumber::Jack,
            12 => CardNumber::Queen,
            13 => CardNumber::King,
            _ => CardNumber::Joker,
        }
    }
}

impl Into<String> for CardNumber {
    fn into(self) -> String {
        match self {
            CardNumber::Ace => "Ace".to_string(),
            CardNumber::Two => "Two".to_string(),
            CardNumber::Three => "Three".to_string(),
            CardNumber::Four => "Four".to_string(),
            CardNumber::Five => "Five".to_string(),
            CardNumber::Six => "Six".to_string(),
            CardNumber::Seven => "Seven".to_string(),
            CardNumber::Eight => "Eight".to_string(),
            CardNumber::Nine => "Nine".to_string(),
            CardNumber::Ten => "Ten".to_string(),
            CardNumber::Jack => "Jack".to_string(),
            CardNumber::Queen => "Queen".to_string(),
            CardNumber::King => "King".to_string(),
            CardNumber::Joker => "Joker".to_string(),
        }
    }
}

impl Display for CardNumber {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cn_string: String = (*self).into();
        write!(f, "{}", cn_string)
    }
}
