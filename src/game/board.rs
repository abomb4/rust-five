use super::Coordination;
use super::ArrayIndex;

const DEFAULT_BOARD_SIZE: usize = 19;

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum BoardPieceType {
    EMPTY, BLACK, WHITE
}


/// A Gomoku game board coordination
///
/// <pre>
///                     x(i)
/// ---------------------->
/// | / 1 2 3 4 5 6 7 8 9
/// | 1 + + + + + + + + +
/// | 2 + + + + + + + + +
/// | 3 + + + + + + + + +
/// | 4 + + + + + + + + +
/// | 5 + + + + + + + + +
/// | 6 + + + + + + + + +
/// | 7 + + + + + + + + +
/// | 8 + + + + + + + + +
/// | 9 + + + + + + + + +
/// v
/// y(j)
/// </pre>
///
pub struct Board {
    /// Stored coordination x-axis and y-axis is reversed.
    board: [[BoardPieceType; DEFAULT_BOARD_SIZE]; DEFAULT_BOARD_SIZE],
    size: usize,
}

fn translate_board_point(target: BoardPieceType) -> &'static str {
    match target {
        BoardPieceType::EMPTY => "Empty",
        BoardPieceType::WHITE => "White",
        BoardPieceType::BLACK => "Black",
    }
}

impl Board {
    /// Create new empty game board
    pub fn new() -> Board {
        const SIZE: usize = DEFAULT_BOARD_SIZE;
        let board: [[BoardPieceType; SIZE ]; SIZE] = [[BoardPieceType::EMPTY; SIZE]; SIZE];

        return Board { board, size: SIZE };
    }

    /// Draw game board to console
    pub fn draw_console(&self) {
        print!("  ");
        let base_a = 'A' as u8;
        for i in 0..self.size {
            print!(" {}", (base_a + i as u8) as char);
        }
        println!();

        // Stored coordination x-axis and y-axis is reversed.
        // So we need print 2nd dimension first
        for j in 0..self.size {
            print!("{:2}", j + 1);
            for i in 0..self.size {
                print!("{}", self.get_board_symbol(i, j));
            }
            println!();
        }
    }

    /// Get a point from board
    ///
    /// x and y starts by 1, not 0
    pub fn get(&self, x: Coordination, y: Coordination) -> Result<BoardPieceType, String> {
        if !self.point_range_check(x, y) {
            return Err(format!("Coordinate ({}, {}) is out of bound.", x, y));
        }

        let i = x - 1;
        let j = y - 1;

        Ok(self.board[i][j])
    }

    /// Place a piece to board
    pub fn place(&mut self, x: Coordination, y: Coordination, point: BoardPieceType) -> Result<BoardPieceType, String> {
        let current_point = match self.get(x, y) {
            Ok(ok) => ok,
            Err(e) => return Err(e)
        };

        if current_point != BoardPieceType::EMPTY {
            return Err(format!("Coordinate ({}, {}) is {}, not empty.", x, y, translate_board_point(current_point)));
        }

        let i = x - 1;
        let j = y - 1;

        self.board[i][j] = point;
        Ok(point)
    }

    /// Check the range of x and y is valid
    fn point_range_check(&self, x: Coordination, y: Coordination) -> bool {
        if x > self.size || x <= 0 {
            return false;
        }

        if y > self.size || y <= 0 {
            return false;
        }

        return true;
    }

    ///
    /// Get board data, translate to console friendly symbol
    ///
    fn get_board_symbol(&self, i: ArrayIndex, j: ArrayIndex) -> &str {
        // i is x-axis, j is y-axis
        let data = self.get(i + 1, j + 1).unwrap();
        // let max_index = self.size - 1;
        // match data {
        //     BoardPoint::WHITE => "○",
        //     BoardPoint::BLACK => "●",
        //     BoardPoint::EMPTY => if i == 0 && j == 0 { "┌" }
        //               else if i == 0 && j == max_index { "└" }
        //               else if i == max_index && j == 0 { "┐" }
        //               else if i == max_index && j == max_index { "┘" }
        //               else if i == 0 { "├" }
        //               else if i == max_index { "┤" }
        //               else if j == 0 { "┬" }
        //               else if j == max_index { "┴" }
        //               else { "┼" }
        //     _ => panic!("Unknown board data detected.")
        // }
        match data {
            BoardPieceType::WHITE => " O",
            BoardPieceType::BLACK => " X",
            BoardPieceType::EMPTY => " -"
        }
    }
}
