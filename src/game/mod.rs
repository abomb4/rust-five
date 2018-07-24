use game::board::Board;

mod board;

pub struct Game {
    board: Board
}

impl Game {

    pub fn new() -> Game{
        Game { board: Board::new() }
    }

    pub fn init(&self) {
        self.board.draw()
    }
}