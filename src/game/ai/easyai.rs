
use game::ai::GomokuAi;
use game::board::Board;

pub struct EasyAI {

}

impl EasyAI {

    pub fn new() -> impl GomokuAi {
        let ai = EasyAI { };
        ai
    }
}

impl GomokuAi for EasyAI {

    fn pointed(&mut self, board: &Board, last_x: isize, last_y: isize) -> (isize, isize) {

        (1, 1)
    }
}
