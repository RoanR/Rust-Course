use cards::Deck;

mod cards;

fn main() {
    let mut deck = Deck::new();
    println!("Here's your deck: {}", deck);

    println!("Shuffling ... ");
    deck.shuffle();

    println!("Dealing ... ");
    let hand = deck.deal(2);

    println!("Here's your hand:");
    for card in hand {
        println!("\t{}", card);
    }
}
