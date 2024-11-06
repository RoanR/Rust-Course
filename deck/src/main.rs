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
    let mut p = Player::new(hand);
    while !p.bust() {
        deck.hit(&mut p);
        println!("\nHitting You!");
        print!("New hand: ");
        for card in p.hand() {
            print!("{}, ", card);
        }
    }
}
