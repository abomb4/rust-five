
pub(super) trait Player {

}

pub(super) struct LocalHumanPlayer {

}

impl LocalHumanPlayer {
    pub fn new() -> impl Player {
        LocalHumanPlayer {}
    }
}

impl Player for LocalHumanPlayer {

}
