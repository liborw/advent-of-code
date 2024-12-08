use std::fmt::Display;


#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {

    pub const ALL: [Direction; 4] = [
        Direction::Up,
        Direction::Right,
        Direction::Down,
        Direction::Left,
    ];

    pub fn turn_right(&self) -> Self {
        use Direction::*;
        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up
        }
    }

    pub fn turn_left(&self) -> Self {
        use Direction::*;
        match self {
            Up => Left,
            Right => Up,
            Down => Right,
            Left => Down
        }
    }

    pub fn oposite(&self) -> Self {
        use Direction::*;
        match self {
            Up => Down,
            Right => Left,
            Down => Up,
            Left => Right
        }
    }
}

impl TryFrom<&char> for Direction {
    type Error = String;
    fn try_from(value: &char) -> Result<Self, Self::Error> {
        use Direction::*;
        match value {
            '^' => Ok(Up),
            '>' => Ok(Right),
            'v' => Ok(Down),
            '<' => Ok(Left),
            c   => Err(format!("Not a direction: {}", c)),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Direction::*;
        let c = match self {
            Up    => '^',
            Right => '>',
            Down  => 'v',
            Left  => '<',
        };
        write!(f, "{}", c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
