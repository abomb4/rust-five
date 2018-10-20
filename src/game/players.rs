
use super::Coordination;
use super::GameContext;
use super::PieceType;

/// Gomoku player trait,
/// class implements this trait should provide a blocking piece pointing method.
pub(super) trait Player {

    /// Blocking method
    fn point(&mut self, context: &GameContext) -> (Coordination, Coordination);

    /// Get what the piece color the player holds
    fn piece_type(&self) -> PieceType;

    /// Print the player common name
    fn name(&self) -> &'static str;
}

/// Local human player
///
/// Currently the game only have console ui, so the player reads stdio input.
pub(super) struct LocalHumanPlayer {
    piece: PieceType
}

impl LocalHumanPlayer {
    pub fn new(piece: PieceType) -> impl Player {
        LocalHumanPlayer { piece }
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

    fn piece_type(&self) -> PieceType {
        self.piece
    }

    fn name(&self) -> &'static str {
        "Human"
    }
}

pub struct IdiotAi {
    piece: PieceType,
    last: (Coordination, Coordination)
}

static mut IDIOTS: usize = 0;
impl IdiotAi {
    pub fn new(piece: PieceType) -> Self {
        let num = unsafe { IdiotAi::get_counter() };
        IdiotAi { piece, last: (num, 0) }
    }

    unsafe fn get_counter() -> usize {
        let num = IDIOTS;
        IDIOTS += 1;
        return num;
    }
}

impl Player for IdiotAi {
    fn point(&mut self, context: &GameContext) -> (Coordination, Coordination) {
        let (x, y) = (self.last.0 + 1, self.last.1 + 1);

        self.last.0 = x;
        self.last.1 = y;

        (x, y)
    }

    fn piece_type(&self) -> PieceType {
        self.piece
    }

    fn name(&self) -> &'static str {
        "Idiot AI"
    }
}
