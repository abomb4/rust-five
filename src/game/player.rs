use super::Coordination;
use super::GameContext;

/// Gomoku player trait,
/// class implements this trait should provide a blocking piece pointing method.
pub(super) trait Player {

    fn point(&mut self, context: &GameContext) -> (Coordination, Coordination);
}

/// Local human player
///
/// Currently the game only have console ui, so the player reads stdio input.
pub(super) struct LocalHumanPlayer {
}

impl LocalHumanPlayer {
    pub fn new() -> impl Player {
        LocalHumanPlayer {}
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
}

impl Player for LocalHumanPlayer {

    fn point(&mut self, context: &GameContext) -> (Coordination, Coordination) {
        LocalHumanPlayer::read_input()
    }
}
