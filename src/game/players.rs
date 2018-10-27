use super::coord::CoordinationFlat;
use super::GameContext;
use super::PieceType;

/// Gomoku player trait,
/// class implements this trait should provide a blocking piece pointing method.
pub(super) trait Player {

    /// Blocking method
    fn point(&mut self, context: &GameContext) -> CoordinationFlat;

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
    fn read_input() -> CoordinationFlat {

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

            return CoordinationFlat::new(x, y);
        }
    }
}

impl Player for LocalHumanPlayer {

    fn point(&mut self, context: &GameContext) -> CoordinationFlat {
        LocalHumanPlayer::read_input()
    }

    fn piece_type(&self) -> PieceType {
        self.piece
    }

    fn name(&self) -> &'static str {
        "Human"
    }
}

use game::board::Board;
use super::Player;
use super::super::coord::CoordinationFlat;
use super::super::GameContext;
use super::super::PieceType;pub(super) mod ai {
    pub struct IdiotAi {
        piece: PieceType,
        last: CoordinationFlat
    }

    static mut IDIOTS: usize = 0;

    impl IdiotAi {
        pub fn new(piece: PieceType) -> Self {
            let num = unsafe { IdiotAi::get_counter() };
            IdiotAi { piece, last: CoordinationFlat::new(num, 0) }
        }

        unsafe fn get_counter() -> usize {
            let num = IDIOTS;
            IDIOTS += 1;
            return num;
        }
    }

    impl Player for IdiotAi {
        fn point(&mut self, context: &GameContext) -> CoordinationFlat {
            let (x, y) = (self.last.x + 1, self.last.y + 1);

            self.last.x = x;
            self.last.y = y;

            CoordinationFlat::new(x, y)
        }

        fn piece_type(&self) -> PieceType {
            self.piece
        }

        fn name(&self) -> &'static str {
            "Idiot AI"
        }
    }

    /// Easy AI, this may be my first game AI implementation
    pub struct EasyAi {
        piece: PieceType
    }

    impl EasyAi {
        pub fn new(piece: PieceType) -> EasyAi {
            EasyAi { piece }
        }

        /// Find which points need calculate
        ///
        /// Points around existing pieces within 4 distance need calculate
        fn find_points_need_calculate(board: &Board) -> Vec<CoordinationFlat> {
            unimplemented!()
        }

        /// Calculate a score at specific point
        fn calculate_score(board: &Board, coord: CoordinationFlat) -> usize {
            unimplemented!()
        }
    }

    impl Player for EasyAi {

        /// Easy AI will point!
        ///
        /// This AI will do this steps:
        /// 1. If the game just start (total points lesser than 5), find templates(?).
        /// 2. If the opponent player not playing as normal (Cannot found template),
        ///    try the most benefit way.
        /// 3. Calculate scores in every places around the existing pieces with 4 distance
        fn point(&mut self, context: &GameContext) -> CoordinationFlat {
            let board = context.board;
            let last = context.last_point;
            let total = context.total_pieces;

            // Found which points should calculate score
            let need_calculate = EasyAi::find_points_need_calculate(&board);

            // Calculate every score

            unimplemented!()
        }

        fn piece_type(&self) -> PieceType {
            self.piece_type()
        }

        fn name(&self) -> &'static str {
            "Easy AI"
        }
    }
}
