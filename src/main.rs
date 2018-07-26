//
// Main method should start a menu?
// Then determine how to initialize a game.
// The game ui using what? sdl?? console??
//

use game::Game;

pub mod game;

fn main() {
    win1();
    win2();
}

fn win1() {
    let mut game = Game::new();
    game.init();
    game.point(1, 5).unwrap();
    game.point(19, 5).unwrap();
    game.point(1, 6).unwrap();
    game.point(19, 6).unwrap();
    game.point(1, 7).unwrap();
    game.point(19, 7).unwrap();
    game.point(1, 8).unwrap();

    let result = game.point(19, 8).unwrap();
    game.draw();
    match result {
        Some(v) => println!("The game is end, winner is {}.", game::translate_player(v)),
        None => println!("The game is not end.")
    }

    let result = game.point(1, 9).unwrap();
    game.draw();
    match result {
        Some(v) => println!("The game is end, winner is {}.", game::translate_player(v)),
        None => println!("The game is not end.")
    }
}

fn win2() {
    let mut game = Game::new();
    game.init();
    game.point(1, 5).unwrap();
    game.point(19, 5).unwrap();
    game.point(1, 6).unwrap();
    game.point(19, 6).unwrap();
    game.point(1, 7).unwrap();
    game.point(19, 7).unwrap();
    game.point(1, 8).unwrap();
    game.point(19, 8).unwrap();

    let result = game.point(1, 1).unwrap();
    game.draw();
    match result {
        Some(v) => println!("The game is end, winner is {}.", game::translate_player(v)),
        None => println!("The game is not end.")
    }

    let result = game.point(19, 9).unwrap();
    game.draw();
    match result {
        Some(v) => println!("The game is end, winner is {}.", game::translate_player(v)),
        None => println!("The game is not end.")
    }
}