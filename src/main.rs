//
// Main method should start a menu?
// Then determine how to initialize a game.
// The game ui using what? sdl?? console??
//

use game::Game;

pub mod game;

fn main() {
    let mut game = Game::new();
    game.init();
    game.point(5, 5);
    game.point(5, 10);
    game.point(10, 10);
    game.point(10, 12);
    game.draw();
}
