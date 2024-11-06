use blackjack::Player;
use cards::Deck;

mod blackjack;
mod cards;

fn main() {
    let mut deck = Deck::new();
    println!("Here's your deck: {}", deck);

    println!("Shuffling ... ");
    deck.shuffle();

    println!("Dealing ... ");
    let hand = deck.deal(2);

    println!("Here's your hand:");
    for card in &hand {
        println!("\t{}", *card);
    }

    println!("Creating a new Player");
    let p = Player::new(hand);
    println!("Are you bust {}", p.bust());
}
