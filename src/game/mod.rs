use game::board::Board;

mod board;

type Player = u8;

/// The two Gomoku player
const BLACK: Player = 0;
const WHITE: Player = 1;


/// Player value to board point value
///
/// <i>This may be a bad design</i>
fn player_to_board_point(p: Player) -> board::BoardPoint {
    match p {
        BLACK => board::BOARD_BLACK,
        WHITE => board::BOARD_WHITE,
        _ => panic!("Unknown player value {}.", p)
    }
}


pub fn translate_player(target: Player) -> &'static str {
    match target {
        WHITE => "White",
        BLACK => "Black",
        _ => panic!("Unknown player {}.", target.clone())
    }
}

///
/// A Gomoku game instance.
///
pub struct Game {
    board: Board,
    current_player: u8,
    history: Vec<(Player, usize, usize)>,
}

impl Game {
    /// Create a new game with black first
    pub fn new() -> Game {
        Game {
            board: Board::new(),
            current_player: BLACK,
            history: vec![]
        }
    }

    /// Draw game graphic
    pub fn draw(&self) {
        self.board.draw()
    }

    /// Initialize the game
    pub fn init(&mut self) {
        self.board.draw()
    }

    /// Start the game!
    pub fn start(&mut self) {}

    /// Place a piece in the game
    ///
    /// Returns the winner if the game is end.
    pub fn point(&mut self, x: isize, y: isize) -> Result<Option<Player>, String> {
        // place the piece to board, and check the game is end
        let place = self.board.place(x, y, player_to_board_point(self.current_player));
        if place.is_err() {
            return Err(place.unwrap_err())
        }

        self.history.push((self.current_player, x as usize, y as usize));

        let winner = if self.check_game_end() {
            Some(self.current_player)
        } else {
            None
        };

        self.current_player = match self.current_player {
            BLACK => WHITE,
            WHITE => BLACK,
            _ => panic!(format!("Invalid player {}.", self.current_player))
        };

        Ok(winner)
    }

    /// Check the game is end, if end, returns true; not end the return false.
    fn check_game_end(&self) -> bool {
        let last_point = match self.history.last() {
            Some(a) => a,
            None => return false
        };
        
        // Current position information
        let last_player_color: board::BoardPoint = player_to_board_point(last_point.0);
        let last_x: isize = last_point.1 as isize;
        let last_y: isize = last_point.2 as isize;

        // Define 4 non-parallel directions
        const MOVE_DIRECITON: [(isize, isize); 4] = [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1)
        ];

        // Check 4 directions negative and positive directions from point position
        for dir in MOVE_DIRECITON.iter() {
            let mut score: isize = 1;

            {
                let mut pointer_x: isize = last_x + dir.0;
                let mut pointer_y: isize = last_y + dir.1;
                let mut a = self.board.get(pointer_x, pointer_y);
                while a.is_ok() && a.unwrap() == last_player_color {
                    score += 1;
                    pointer_x += dir.0;
                    pointer_y += dir.1;
                    a = self.board.get(pointer_x, pointer_y);
                }
            }

            {
                let mut pointer_x: isize = last_x - dir.0;
                let mut pointer_y: isize = last_y - dir.1;
                let mut a = self.board.get(pointer_x, pointer_y);
                while a.is_ok() && a.unwrap() == last_player_color {
                    score += 1;
                    pointer_x -= dir.0;
                    pointer_y -= dir.1;
                    a = self.board.get(pointer_x, pointer_y);
                }
            }

            if score >= 5 {
                return true;
            }
        }

        false
    }
}