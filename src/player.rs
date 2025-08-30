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

impl From<usize> for Player {
    fn from(value: usize) -> Self {
        match value {
            0 => Player::Black,
            1 => Player::White,
            _ => panic!("Invalid player value: {}", value),
        }
    }
}

impl From<Player> for usize {
    fn from(player: Player) -> Self {
        player as usize
    }
}
