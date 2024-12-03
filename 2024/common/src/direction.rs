use std::fmt::Display;





pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {
    // Turn in counter clockwise direction
    pub fn turn_cc(&self) -> Self {
        match &self {
            Direction::Up => Direction::Left,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
            Direction::Right => Direction::Up
        }
    }

    // Turn in clockwise direction
    pub fn turn_c(&self) -> Self {
        match &self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down
        }
    }

}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match &self {
            Direction::Up => '^',
            Direction::Down => 'v',
            Direction::Left => '<',
            Direction::Right => '>',
        };
        write!(f, "{}", char)
    }
}
