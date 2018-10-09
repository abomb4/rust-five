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
    main_loop_console(&mut game);
}

/// Main game loop until game is end
fn main_loop_console(game: &mut Game) {

    loop {
        // Read input from user
        let input = read_input();
        let (x, y) = input;

        // Try point the coordinate
        let optional_winner = match game.point(x, y) {
            Ok(v) => v,
            Err(e) => { println!("Failed point to ({}, {}), {}", x, y, e); continue; }
        };

        // Print
        game.draw();

        // See if there is a winner.
        match optional_winner {
            Some(v) => { println!("Winner is {}.", game::translate_player(v)); break; },
            None => { }
        };

    }
}

/// Loop get user coordinate input
fn read_input() -> (usize, usize) {

    use std::io::{ stdin, stdout, Write };
    use std::usize;

    loop {
        let mut s = String::new();

        print!("Input the coordinate(x and y, like j10 or i9):");
        let _ = stdout().flush();

        stdin().read_line(&mut s).expect("Did not enter a correct string.");
        if let Some('\n') = s.chars().next_back() {
            s.pop();
        }
        if let Some('\r') = s.chars().next_back() {
            s.pop();
        }

        let x_str = &s[0..1];
        let y_str = &s[1..];

        println!("x_str is {}, y_str is {}", x_str, y_str);

        let x = match usize::from_str_radix(x_str, 36) {
            Ok(v) => v - 9,
            Err(_e) => {
                println!("Invalid X input [{}]", s);
                continue;
            }
        };

        let y = match usize::from_str_radix(y_str, 10) {
            Ok(v) => v,
            Err(_e) => {
                println!("Invalid Y input [{}]", s);
                continue;
            }
        };

        return (x, y);
    }
}
