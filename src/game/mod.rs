use game::PieceType::BLACK;
use game::PieceType::WHITE;
use game::players::ai::IdiotAi;
use self::board::Board;
use self::coord::CoordinationFlat;
use self::players::LocalHumanPlayer;
use self::players::Player;
use std::char;
use std::fmt;

mod board;
mod players;

mod coord {
    use std::fmt;

    /// Define coordination type
    type Coordination = usize;

    // 2D Coordination
    #[derive(Copy, Clone)]
    pub struct CoordinationFlat {
        pub x: Coordination,
        pub y: Coordination
    }

    impl CoordinationFlat {
        pub fn new(x: Coordination, y: Coordination) -> CoordinationFlat {
            CoordinationFlat { x, y }
        }
    }
    impl fmt::Display for CoordinationFlat {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "({}, {})", self.x, self.y)
        }
    }
}

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

    pub fn to_board_piece_type(&self) -> board::BoardPieceType {
        match self {
            PieceType::BLACK => board::BoardPieceType::BLACK,
            PieceType::WHITE => board::BoardPieceType::WHITE,
        }
    }
}

impl fmt::Display for PieceType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.get_name())
    }
}


#[derive(Copy, Clone, Eq, PartialEq)]
pub enum GameBuilderPlayerType {
    Human,
    IdiotAi,
}
/// Game builder
pub struct GameBuilder {
    first_player: GameBuilderPlayerType,
    second_player: GameBuilderPlayerType
}

impl GameBuilder {

    /// Create an game builder object
    pub fn new() -> GameBuilder {
        GameBuilder {
            first_player: GameBuilderPlayerType::Human,
            second_player: GameBuilderPlayerType::Human
        }
    }

    /// Set the first player (Uses black piece)
    pub fn set_first_player(&mut self, player_type: GameBuilderPlayerType) -> &mut Self {
        self.first_player = player_type;
        self
    }

    /// Set the second player (Uses black piece)
    pub fn set_second_player(&mut self, player_type: GameBuilderPlayerType) -> &mut Self {
        self.second_player = player_type;
        self
    }

    pub fn build(&self) -> Game {
        Game::new(
            GameBuilder::create_player(self.first_player, BLACK),
            GameBuilder::create_player(self.second_player, WHITE),
        )
    }

    fn create_player(player_type: GameBuilderPlayerType, piece: PieceType) -> Box<Player> {
        match player_type {
            GameBuilderPlayerType::Human => Box::new(LocalHumanPlayer::new(piece)),
            GameBuilderPlayerType::IdiotAi => Box::new(IdiotAi::new(piece))
        }
    }
}

///
/// Game context in game, typically is same as Game struct
///
pub(in game) struct GameContext {
    /// A board 2D array copy
    board: Board,
    /// None if it's first player point
    last_point: Option<CoordinationFlat>,
    /// Total pieces in the game
    total_pieces: usize
}

impl GameContext {

    pub fn new(board: Board, last_point: Option<CoordinationFlat>, total_pieces: usize)
        -> Self {
        GameContext {
            board,
            last_point,
            total_pieces
        }
    }
}

///
/// A Gomoku game instance.
///
pub struct Game {
    board: Board,
    players: [Box<Player>; 2],
    current_player: usize,
    // TODO Can history put reference of player into?
    history: Vec<(PieceType, CoordinationFlat)>,
    started: bool,
    ended: bool,
}

impl Game {
    /// Create a new game with black first
    fn new(first_player: Box<Player>, second_player: Box<Player>) -> Game {

        Game {
            board: Board::new(),
            current_player: 0,
            players: [first_player, second_player],
            history: vec![],
            started: false,
            ended: false,
        }
    }

    /// Create an game builder object, equals with GameBuilder::new()
    pub fn game_builder() -> GameBuilder {
        GameBuilder::new()
    }

    /// Start the game!
    ///
    /// This function will initialize the game,
    /// and start main game loop.
    pub fn start(&mut self) {
        self.init();
        self.started = true;
        self.main_loop();
    }

    /// Initialize the game.
    ///
    /// This function will initialize the game board,
    /// but currently is unreusable, so that is not needed.
    ///
    /// Currently for the console version Gomoku game,
    /// this method prints the game board to console.
    fn init(&mut self) {
        self.draw();
    }

    /// Draw game graphic
    fn draw(&self) {
        println!();
        self.board.draw_console();
        if !self.ended {
            self.print_player();
        }
    }

    /// Print who will point this time
    fn print_player(&self) {
        let p = self.get_current_player();
        print!("{} ({}) turn to point: ", p.name(), p.piece_type().get_name());
    }

    /// Print where is pointed
    fn print_point(&self, coord: CoordinationFlat) {
        let x = coord.x;
        let y = coord.y;
        let char_x = char::from_digit((x + 9) as u32, 36).unwrap();
        print!("{}{}", char_x, y);
    }

    /// Start the game main loop, loop the two player to point, until the game is end.
    ///
    /// In the loop, when every player placed a piece, the game updates it's board and print,
    /// then invoke the blocking function `Player::point()`, let another place piece.
    fn main_loop(&mut self) {

        let mut fail_count = 0;
        loop {
            // Initialize the game context every lap
            // TODO Is there a better way to references the board?
            let context = GameContext::new(self.board.clone(),
                                           self.history.last().map(|z| { z.1 }),
                                           self.history.len());

            // Read input from player
            let coord = self.get_current_player_mut().point(&context);

            // Try point the coordinate
            let optional_winner = match self.point(coord) {
                Ok(v) => v,
                Err(e) => {
                    fail_count += 1;
                    println!("Failed point to ({}, {}), {}", coord.x, coord.y, e);

                    // Panic if too many invalid point
                    if fail_count >= 6 {
                        panic!("Fail to point 6 times, may due to invalid AI implementation, panic")
                    }
                    continue;
                }
            };

            // Print
            self.print_point(coord);
            self.draw();

            // See if there is a winner.
            match optional_winner {
                Some(_) => {
                    // Current player cannot point anything because another player is wined
                    let winner = self.get_another_player();
                    println!("Winner is {} ({}).", winner.name(), winner.piece_type());
                    break;
                },
                None => { }
            };

            fail_count = 0;
        }
    }

    // TODO Can I returns the reference of winner player?
    /// Place a piece in the game
    ///
    /// Returns the winner if the game is end.
    fn point(&mut self, coord: CoordinationFlat) -> Result<Option<PieceType>, String> {
        if !self.started {
            return Err(String::from("The game has not started yet"))
        }
        if self.ended {
            return Err(String::from("The game is over"))
        }

        // place the piece to board, and check the game is end
        let current_piece = self.get_current_player().piece_type();
        let place = self.board.place(coord, current_piece.to_board_piece_type());
        if place.is_err() {
            return Err(place.err().unwrap())
        }

        self.history.push((current_piece, coord));

        let winner = if self.check_game_end() {
            self.ended = true;
            Some(current_piece)
        } else {
            None
        };

        self.change_to_another_player();

        Ok(winner)
    }

    // Change current player to another player, and returns new current player.
    fn change_to_another_player(&mut self) -> &Box<Player> {
        if self.current_player == 0 {
            self.current_player = 1
        } else {
            self.current_player = 0
        }
        self.get_current_player()
    }

    /// Get another player, don't change the current player state
    fn get_another_player(&self) -> &Box<Player> {
        if self.current_player == 0 {
            &self.players[1]
        } else {
            &self.players[0]
        }
    }

    /// Get another player mutable reference, don't change the current player state
    fn get_another_player_mut(&mut self) -> &mut Box<Player> {
        if self.current_player == 0 {
            &mut self.players[1]
        } else {
            &mut self.players[0]
        }
    }

    /// Get the current player
    fn get_current_player(&self) -> &Box<Player> {
        &self.players[self.current_player]
    }

    /// Get the current player mutable reference
    fn get_current_player_mut(&mut self) -> &mut Box<Player> {
        &mut self.players[self.current_player]
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
        let last_player_color: board::BoardPieceType = last_point.0.to_board_piece_type();
        let last_coordination = last_point.1;

        // Define 4 non-parallel directions
        const MOVE_DIRECTION: [(isize, isize); 4] = [
            (0, 1),
            (1, 1),
            (1, 0),
            (1, -1)
        ];

        fn move_dir(coord: &CoordinationFlat, dir: &(isize, isize)) -> Result<CoordinationFlat, &'static str> {
            let new_x = (coord.x as isize) + dir.0;
            let new_y = (coord.y as isize) + dir.1;

            if new_x < 0 {
                return Err("x is out of bound");
            } else if new_y < 0 {
                return Err("y is out of bound");
            }

            Ok(CoordinationFlat::new(new_x as usize, new_y as usize))
        }

        fn move_dir_reverse(coord: &CoordinationFlat, dir: &(isize, isize)) -> Result<CoordinationFlat, &'static str> {
            let new_x = (coord.x as isize) - dir.0;
            let new_y = (coord.y as isize) - dir.1;

            if new_x < 0 {
                return Err("x is out of bound")
            } else if new_y < 0 {
                return Err("y is out of bound")
            }

            Ok(CoordinationFlat::new(new_x as usize, new_y as usize))
        }

        // Check 4 directions negative and positive directions from point position
        for dir in MOVE_DIRECTION.iter() {
            let mut score = 1;

            {
                let mut next_coord = move_dir(&last_coordination, dir);
                if next_coord.is_ok() {
                    let mut a = self.board.get(next_coord.unwrap());
                    while next_coord.is_ok() && a.is_ok() && a.unwrap() == last_player_color {
                        score += 1;
                        next_coord = move_dir(&next_coord.unwrap(), dir);
                        a = self.board.get(next_coord.unwrap());
                    }
                }
            }

            {
                let mut next_coord = move_dir_reverse(&last_coordination, dir);
                if next_coord.is_ok() {
                    let mut a = self.board.get(next_coord.unwrap());
                    while next_coord.is_ok() && a.is_ok() && a.unwrap() == last_player_color {
                        score += 1;
                        next_coord = move_dir_reverse(&next_coord.unwrap(), dir);
                        a = self.board.get(next_coord.unwrap());
                    }
                }
            }

            if score >= 5 {
                return true;
            }
        }

        false
    }
}
