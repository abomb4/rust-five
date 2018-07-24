const BOARD_SIZE: usize = 19;

/// A Gomoku game board
pub(in game) struct Board {
    board: [[usize; BOARD_SIZE]; BOARD_SIZE]
}

pub const BOARD_NONE: usize = 0;
pub const BOARD_WHITE: usize = 1;
pub const BOARD_BLACK: usize = 2;

impl Board {
    /// Create new empty game board
    pub fn new() -> Board {
        let board: [[usize; BOARD_SIZE]; BOARD_SIZE] = [[BOARD_NONE; BOARD_SIZE]; BOARD_SIZE];

        return Board { board };
    }

    /// Draw game board to console
    pub fn draw(&self) {
        for i in 0..BOARD_SIZE {
            for j in 0..BOARD_SIZE {
                print!("{}", self.get_board_symbol(i, j));
            }
            println!();
        }
    }

    ///
    /// Get board data, translate to console friendly symbol
    ///
    fn get_board_symbol(&self, i: usize, j: usize) -> &str {
        let data = self.board[i][j];
        let max_index = BOARD_SIZE - 1;
        match data {
            BOARD_WHITE => "○",
            BOARD_BLACK => "●",
            BOARD_NONE => if i == 0 && j == 0 { "┌" } else if i == 0 && j == max_index { "┐" } else if i == max_index && j == 0 { "└" } else if i == max_index && j == max_index { "┘" } else if i == 0 { "┬" } else if i == max_index { "┴" } else if j == 0 { "├" } else if j == max_index { "┤" } else { "┼" }
            _ => panic!("Unknown board data detected.")
        }
    }
}
