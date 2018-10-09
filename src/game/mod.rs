use self::board::Board;
use self::ai::GomokuAi;
use self::ai::easyai::EasyAI;
use self::player::Player;
use self::player::LocalHumanPlayer;
use std::fmt;

mod board;
mod player;
pub mod ai;

/// Define coordination type
pub type Coordination = usize;
/// Define array index type
pub type ArrayIndex = usize;

/// The Piece type includes black and white
#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PieceType {
    WHITE, BLACK
}

impl PieceType {
    pub fn get_name(&self) -> &str {
        match self {
            PieceType::WHITE => "White",
            PieceType::BLACK => "Black"
        }
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_name())
    }
}

/// Player value to board point value
///
/// <i>This may be a bad design</i>
fn player_to_board_point(p: PieceType) -> board::BoardPieceType {
    match p {
        PieceType::BLACK => board::BoardPieceType::BLACK,
        PieceType::WHITE => board::BoardPieceType::WHITE,
    }
}

/// Translate player code to White or Black
pub fn translate_player(target: PieceType) -> &'static str {
    match target {
        PieceType::WHITE => "White",
        PieceType::BLACK => "Black",
    }
}

/// Available Gomoku AI Types
pub enum GomokuAiType {
    None,
    EasyAi,
}

/// Game builder
pub struct GameBuilder {
    first: PieceType,
    ai: GomokuAiType
}

impl GameBuilder {

    /// Create an game builder object
    pub fn new() -> GameBuilder {
        GameBuilder {
            first: PieceType::BLACK,
            ai: GomokuAiType::None
        }
    }

    /// Set the player who will point first
    pub fn set_first(&mut self, piece: PieceType) -> &Self {
        self.first = piece;
        self
    }

    /// Set the player who will point first
    pub fn select_ai(&mut self, ai: GomokuAiType) -> &Self {
        self.ai = ai;
        self
    }

    pub fn build(&self) -> Game {
        Game::new(self.first, GameBuilder::get_ai(&self.ai))
    }

    fn get_ai(ai: &GomokuAiType) -> Option<Box<GomokuAi>> {
        match ai {
            GomokuAiType::EasyAi => Some(Box::new(EasyAI::new())),
            GomokuAiType::None => None,
        }
    }
}

///
/// A Gomoku game instance.
///
pub struct Game {
    board: Board,
    ai: Option<Box<GomokuAi>>,
    players: [Box<Player>; 2],
    current_piece: PieceType,
    history: Vec<(PieceType, Coordination, Coordination)>,
    started: bool,
    ended: bool,
}

impl Game {
    /// Create a new game with black first
    pub fn new(first_player: PieceType, ai: Option<Box<GomokuAi>>) -> Game {
        Game {
            board: Board::new(),
            ai,
            current_piece: first_player,
            players: [Box::new(LocalHumanPlayer::new()), Box::new(LocalHumanPlayer::new())],
            history: vec![],
            started: false,
            ended: false,
        }
    }

    /// Create an game builder object, equals with GameBuilder::new()
    pub fn game_builder() -> GameBuilder {
        GameBuilder::new()
    }

    /// Draw game graphic
    pub fn draw(&self) {
        self.board.draw_console()
    }

    /// Start the game!
    pub fn start(&mut self) {
        self.init();
        self.started = true
    }

    /// Initialize the game
    fn init(&mut self) {
        self.board.draw_console()
    }

    /// Place a piece in the game
    ///
    /// Returns the winner if the game is end.
    pub fn point(&mut self, x: Coordination, y: Coordination) -> Result<Option<PieceType>, String> {
        if !self.started {
            return Err(String::from("The game has not started yet"))
        }
        if self.ended {
            return Err(String::from("The game is over"))
        }
        // place the piece to board, and check the game is end
        let place = self.board.place(x, y, player_to_board_point(self.current_piece));
        if place.is_err() {
            return Err(place.err().unwrap())
        }

        self.history.push((self.current_piece, x, y));

        let winner = if self.check_game_end() {
            self.ended = true;
            Some(self.current_piece)
        } else {
            None
        };

        self.current_piece = match self.current_piece {
            PieceType::BLACK => PieceType::WHITE,
            PieceType::WHITE => PieceType::BLACK,
        };

        Ok(winner)
    }

    /// Check the game is end, if end, returns true; not end the return false.
    ///
    /// So the winner is the top of history stack
    fn check_game_end(&self) -> bool {
        let last_point = match self.history.last() {
            Some(a) => a,
            None => return false
        };
        
        // Current position information
        let last_player_color: board::BoardPieceType = player_to_board_point(last_point.0);
        let last_x = last_point.1 as isize;
        let last_y = last_point.2 as isize;

        // Define 4 non-parallel directions
        const MOVE_DIRECTION: [(isize, isize); 4] = [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1)
        ];

        // Check 4 directions negative and positive directions from point position
        for dir in MOVE_DIRECTION.iter() {
            let mut score = 1;

            {
                let mut pointer_x = (last_x + dir.0) as Coordination;
                let mut pointer_y = (last_y + dir.1) as Coordination;
                let mut a = self.board.get(pointer_x, pointer_y);
                while a.is_ok() && a.unwrap() == last_player_color {
                    score += 1;
                    pointer_x = (pointer_x as isize + dir.0) as Coordination;
                    pointer_y = (pointer_y as isize + dir.1) as Coordination;
                    a = self.board.get(pointer_x, pointer_y);
                }
            }

            {
                let mut pointer_x = (last_x - dir.0) as Coordination;
                let mut pointer_y = (last_y - dir.1) as Coordination;
                let mut a = self.board.get(pointer_x, pointer_y);
                while a.is_ok() && a.unwrap() == last_player_color {
                    score += 1;
                    pointer_x = (pointer_x as isize - dir.0) as Coordination;
                    pointer_y = (pointer_y as isize - dir.1) as Coordination;
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
