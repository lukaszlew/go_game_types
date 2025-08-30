use crate::Player;
use std::convert::TryFrom;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum Color {
    Black = 0,
    White = 1,
    Empty = 2,
    OffBoard = 3,
}

impl Color {
    pub const COUNT: usize = 4;
}

impl From<Player> for Color {
    fn from(player: Player) -> Self {
        match player {
            Player::Black => Color::Black,
            Player::White => Color::White,
        }
    }
}

impl TryFrom<Color> for Player {
    type Error = &'static str;

    fn try_from(color: Color) -> Result<Self, Self::Error> {
        match color {
            Color::Black => Ok(Player::Black),
            Color::White => Ok(Player::White),
            Color::Empty => Err("Empty is not a player color"),
            Color::OffBoard => Err("OffBoard is not a player color"),
        }
    }
}