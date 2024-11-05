use rand::{seq::SliceRandom, thread_rng};

#[derive(Debug)]
struct Deck {
    cards: Vec<Card>,
}

impl Deck {
    fn new() -> Self {
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

    fn shuffle(&mut self) {
        let mut rng = thread_rng();
        self.cards.shuffle(&mut rng);
    }

    fn deal(&mut self, num_cards: usize) -> Vec<Card> {
        self.cards.split_off(self.cards.len() - num_cards)
    }
}

#[derive(Debug)]
struct Card {
    suit: Suits,
    number: CardNumber,
}

#[derive(Debug, Clone, Copy)]
enum Suits {
    Spades,
    Hearts,
    Clubs,
    Diamonds,
}

#[derive(Debug)]
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

fn main() {
    let mut deck = Deck::new();
    println!("Here's your deck: {:#?}", deck);

    println!("Shuffling ... ");
    deck.shuffle();

    println!("Dealing ... ");
    let hand = deck.deal(2);

    println!("Here's your hand: {:#?}", hand)
}
