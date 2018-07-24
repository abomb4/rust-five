use game::board::Board;

mod board;

/// The two Gomoku player
pub enum Player {
    BLACK,
    WHITE
}

///
/// A Gomoku game instance.
///
pub struct Game {
    board: Board
}

impl Game {

    /// Create a new game
    pub fn new() -> Game{
        Game { board: Board::new() }
    }

    /// Initialize the game
    pub fn init(&self) {
        self.board.draw()
    }

    /// Start the game!
    pub fn start(&self) {}

    /// Place a piece in the game
    ///
    /// Returns the winner if the game is end.
    pub fn point(&self, x: usize, y: usize) -> Option<Player> {
        // TODO place the piece to board, and check the game is end
    }
}