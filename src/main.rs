mod game;
pub mod hands;

use game::Game;

fn main() {
    let mut game = Game::new();
    game.play_game();
}
