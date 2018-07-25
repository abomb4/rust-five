use game::board::Board;

mod board;

type Player = u8;

/// The two Gomoku player
const BLACK: Player = 0;
const WHITE: Player = 1;

fn translate_to_board_type(p: Player) -> board::BoardPoint {
    match p {
        BLACK => board::BOARD_BLACK,
        WHITE => board::BOARD_WHITE,
        _ => panic!(format!("Invalid player {}.", p))
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

    /// Initialize the game
    pub fn init(&mut self) {
        self.board.draw()
    }

    /// Start the game!
    pub fn start(&mut self) {}

    /// Place a piece in the game
    ///
    /// Returns the winner if the game is end.
    pub fn point(&mut self, x: usize, y: usize) -> Result<Option<Player>, String> {
        // place the piece to board, and check the game is end
        let place = self.board.place(x, y, translate_to_board_type(self.current_player));
        if place.is_err() {
            return Err(place.unwrap_err())
        }

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

        match last_point.0 {
            BLACK => true,
            _ => false
        }

    }
}