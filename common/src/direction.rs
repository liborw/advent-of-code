use crate::pos::Pos;


#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Hash)]
pub enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    pub fn move_pos(&self, pos: &Pos) -> Pos {
        match self {
            Direction::Up    => *pos + (-1, 0).into(),
            Direction::Down  => *pos + ( 1, 0).into(),
            Direction::Left  => *pos + ( 0,-1).into(),
            Direction::Right => *pos + ( 0, 1).into(),
        }
    }

    pub fn rotate_left(&self) -> Self {
        match self {
            Direction::Up    => Direction::Left,
            Direction::Down  => Direction::Right,
            Direction::Left  => Direction::Down,
            Direction::Right => Direction::Up,
        }
    }

    pub fn rotate_right(&self) -> Self {
        match self {
            Direction::Up    => Direction::Right,
            Direction::Down  => Direction::Left,
            Direction::Left  => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }
}
