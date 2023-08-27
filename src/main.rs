mod game;
mod hands;

use game::Game;

/// Create a game object to run the game
fn main() {
    let mut game = Game::new();
    game.play_game();
}
