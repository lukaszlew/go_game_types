#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Player {
    Black = 0,
    White = 1,
}

impl Player {
    pub const COUNT: usize = 2;

    pub fn opponent(self) -> Player {
        match self {
            Player::Black => Player::White,
            Player::White => Player::Black,
        }
    }
}
