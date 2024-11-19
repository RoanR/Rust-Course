use blackjack::Game;

mod blackjack;
mod cards;
mod terminal;

fn main() {
    let mut game = Game::new();
    println!("{}", game);
    game.turn();
    game.cpu_turn();
    if game.human_win() {
        println!("THE HUMAN WON!");
    } else {
        println!("the human lost");
    }
}
