use blackjack::Game;

mod blackjack;
mod cards;

fn main() {
    let mut game = Game::new();
    game.turn();
}
