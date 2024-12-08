use std::fmt::Display;

#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
pub enum Direction {
    North,
    NorthEast,
    East,
    SouthEast,
    South,
    SouthWest,
    West,
    NorthWest,
}

impl Direction {

    pub const ALL: [Direction; 8] = [
        Direction::North,
        Direction::NorthEast,
        Direction::East,
        Direction::SouthEast,
        Direction::South,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest,
    ];

    pub fn turn_right(&self) -> Self {
        use Direction::*;
        match self {
            North => NorthEast,
            NorthEast => East,
            East => SouthEast,
            SouthEast => South,
            South => SouthWest,
            SouthWest => West,
            West => NorthWest,
            NorthWest => North
        }
    }

    pub fn turn_left(&self) -> Self {
        use Direction::*;
        match self {
            North => NorthWest,
            NorthWest => West,
            West => SouthWest,
            SouthWest => South,
            South => SouthEast,
            SouthEast => East,
            East => NorthEast,
            NorthEast => North
        }
    }

    pub fn oposite(&self) -> Self {
        use Direction::*;
        match self {
            North => South,
            NorthEast => SouthWest,
            East => West,
            SouthEast => NorthWest,
            South => North,
            SouthWest => NorthEast,
            West => East,
            NorthWest => SouthEast
        }
    }
}

impl TryFrom<&char> for Direction {
    type Error = String;
    fn try_from(value: &char) -> Result<Self, Self::Error> {
        use Direction::*;
        match value {
            '^' => Ok(North),
            '>' => Ok(East),
            'v' => Ok(South),
            '<' => Ok(West),
            c   => Err(format!("Not a direction: {}", c)),
        }
    }
}

impl Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Direction::*;
        let c = match self {
            North => "N",
            NorthEast => "NE",
            East => "E",
            SouthEast => "SE",
            South => "S",
            SouthWest => "SW",
            West => "W",
            NorthWest => "NW"
        };
        write!(f, "{}", c)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}
