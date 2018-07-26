const DEFAULT_BOARD_SIZE: isize = 19;

pub(in game) type BoardPoint = isize;

/// A Gomoku game board
///
/// <pre>
///                     x(i)
/// ---------------------->
/// | 0 1 2 3 4 5 6 7 8 9
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
pub(in game) struct Board {
    /// Stored coordination x-axis and y-axis is reversed.
    board: [[isize; DEFAULT_BOARD_SIZE as usize]; DEFAULT_BOARD_SIZE as usize],
    size: isize,
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
        const SIZE: usize = DEFAULT_BOARD_SIZE as usize;
        let board: [[isize; SIZE ]; SIZE] = [[BOARD_EMPTY; SIZE]; SIZE];

        return Board { board, size: SIZE as isize };
    }

    /// Draw game board to console
    pub fn draw(&self) {

        // Stored coordination x-axis and y-axis is reversed.
        // So we need print 2nd dimension first
        for j in 0..self.size {
            for i in 0..self.size {
                print!("{}", self.get_board_symbol(i as isize, j as isize));
            }
            println!();
        }
    }

    /// Get a point from board
    ///
    /// x and y starts by 1, not 0
    pub fn get(&self, x: isize, y: isize) -> Result<BoardPoint, String> {
        if !self.point_range_check(x, y) {
            return Err(format!("Coordinate ({}, {}) is out of bound.", x, y));
        }

        let i: isize = x - 1;
        let j: isize = y - 1;

        Ok(self.board[i as usize][j as usize])
    }

    /// Place a piece to board
    pub fn place(&mut self, x: isize, y: isize, point: BoardPoint) -> Result<BoardPoint, String> {
        let current_point = match self.get(x, y) {
            Ok(ok) => ok,
            Err(e) => return Err(e)
        };

        if current_point != BOARD_EMPTY {
            return Err(format!("Coordinate ({}, {}) is {}, not empty.", x, y, translate_board_point(current_point)));
        }

        let i = x - 1;
        let j = y - 1;

        self.board[i as usize][j as usize] = point;
        Ok(point)
    }

    /// Check the range of x and y is valid
    fn point_range_check(&self, x: isize, y: isize) -> bool {
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
    fn get_board_symbol(&self, i: isize, j: isize) -> &str {
        // i is x-axis, j is y-axis
        let data = self.get(i + 1, j + 1).unwrap();
        let max_index = self.size - 1;
        match data {
            BOARD_WHITE => "○",
            BOARD_BLACK => "●",
            BOARD_EMPTY => if i == 0 && j == 0 { "┏" }
                      else if i == 0 && j == max_index { "┗" }
                      else if i == max_index && j == 0 { "┓" }
                      else if i == max_index && j == max_index { "┛" }
                      else if i == 0 { "┣" }
                      else if i == max_index { "┫" }
                      else if j == 0 { "┳" }
                      else if j == max_index { "┻" }
                      else { "╋" }
            _ => panic!("Unknown board data detected.")
        }
    }
}
