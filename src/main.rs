//
// Main method should start a menu?
// Then determine how to initialize a game.
// The game ui using what? sdl?? console??
//

use game::Game;

pub mod game;

fn main() {
    let mut game = Game::game_builder().build();

    game.start();
}
