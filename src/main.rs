//
// Main method should start a menu?
// Then determine how to initialize a game.
// The game ui using what? sdl?? console??
//

use game::Game;
use game::GameBuilderPlayerType::Human;
use game::GameBuilderPlayerType::IdiotAi;

pub mod game;

fn main() {
    let mut builder = Game::game_builder();
    builder
        .set_first_player(IdiotAi)
        .set_second_player(IdiotAi);

    let mut game = builder.build();

    game.start();
}
