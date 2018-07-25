const DEFAULT_BOARD_SIZE: usize = 19;

pub(in game) type BoardPoint = usize;

/// A Gomoku game board
pub(in game) struct Board {
    board: [[usize; DEFAULT_BOARD_SIZE]; DEFAULT_BOARD_SIZE],
    size: usize,
}

pub const BOARD_EMPTY: BoardPoint = 0;
pub const BOARD_WHITE: BoardPoint = 1;
pub const BOARD_BLACK: BoardPoint = 2;

fn translate_board_point(target: BoardPoint) -> &'static str {
    match target {
        BOARD_EMPTY => "Empty",
        BOARD_WHITE => "White",
        BOARD_BLACK => "Black",
        _ => panic!("Unknown board point {}.", target.clone())
    }
}

impl Board {
    /// Create new empty game board
    pub fn new() -> Board {
        const SIZE: usize = DEFAULT_BOARD_SIZE;
        let board: [[usize; SIZE]; SIZE] = [[BOARD_EMPTY; SIZE]; SIZE];

        return Board { board, size: SIZE };
    }

    /// Draw game board to console
    pub fn draw(&self) {
        for i in 0..self.size {
            for j in 0..self.size {
                print!("{}", self.get_board_symbol(i, j));
            }
            println!();
        }
    }

    /// Get a point from board
    ///
    /// x and y starts by 1, not 0
    pub fn get(&self, x: usize, y: usize) -> Result<BoardPoint, String> {
        if !self.point_range_check(x, y) {
            return Err(format!("Coordinate ({}, {}) is out of bound.", x, y));
        }

        let i = x - 1;
        let j = y - 1;

        Ok(self.board[i][j])
    }

    /// Place a piece to board
    pub fn place(&mut self, x: usize, y: usize, point: BoardPoint) -> Result<BoardPoint, String> {
        let current_point = match self.get(x, y) {
            Ok(ok) => ok,
            Err(e) => return Err(e)
        };

        if current_point != BOARD_EMPTY {
            return Err(format!("Coordinate ({}, {}) is {}, not empty.",x, y, translate_board_point(current_point)))
        }

        let i = x - 1;
        let j = y - 1;

        self.board[i][j] = point;
        Ok(point)
    }

    /// Check the range of x and y is valid
    fn point_range_check(&self, x: usize, y: usize) -> bool {
        if x >= self.size || x <= 0 {
            return false;
        }

        if y >= self.size || y <= 0 {
            return false;
        }

        return true;
    }

    ///
    /// Get board data, translate to console friendly symbol
    ///
    fn get_board_symbol(&self, i: usize, j: usize) -> &str {
        let data = self.board[i][j];
        let max_index = self.size - 1;
        match data {
            BOARD_WHITE => "○",
            BOARD_BLACK => "●",
            BOARD_EMPTY => if i == 0 && j == 0 { "┌" } else if i == 0 && j == max_index { "┐" } else if i == max_index && j == 0 { "└" } else if i == max_index && j == max_index { "┘" } else if i == 0 { "┬" } else if i == max_index { "┴" } else if j == 0 { "├" } else if j == max_index { "┤" } else { "┼" }
            _ => panic!("Unknown board data detected.")
        }
    }
}
