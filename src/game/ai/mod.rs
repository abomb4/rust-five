
use game::board::Board;
use game::ai::easyai::EasyAI;

pub(super) mod easyai;

pub trait GomokuAi {

    // Call after a point
    fn pointed(&mut self, board: &Board, last_x: isize, last_y: isize) -> (isize, isize);
}

pub fn get_gomoku_ai() -> impl GomokuAi {
    EasyAI::new()
}
