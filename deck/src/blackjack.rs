use crate::cards::{Card, Deck};

pub struct Game {
    deck: Deck,
    human: Player,
    computer: Player,
    turn: bool,
}

impl Game {
    pub fn new() -> Self {
        println!("Starting new game of BlackJack");
        let mut deck = Deck::new();
        println!("Shuffling and Dealing ..... ");
        deck.shuffle();
        let computer = Player::new(deck.deal(2));
        let human = Player::new(deck.deal(2));
        println!(
            "Dealers showing: \n\tFacedown Card\n\t{}",
            computer.hand()[1]
        );
        println!("Your cards: \n\t{}\n\t{}", human.hand()[0], human.hand()[1]);
        Self {
            deck,
            human,
            computer,
            turn: true,
        }
    }
}
pub struct Player {
    hand: Vec<Card>,
}

impl Player {
    pub fn new(hand: Vec<Card>) -> Self {
        Self { hand }
    }

    pub fn hand(&self) -> &[Card] {
        &self.hand
    }

    pub fn bust(&self) -> bool {
        let mut total = 0;
        for card in &self.hand {
            if card.value() > 10 {
                total += 10;
            } else {
                total += card.value();
            }
        }
        total > 21
    }

    pub fn hit(&mut self, card: Card) {
        self.hand.push(card);
    }
}

impl Deck {
    pub fn hit(&mut self, player: &mut Player) {
        Deck::shuffle(self);
        player.hit(Deck::deal(self, 1)[0]);
    }
}
