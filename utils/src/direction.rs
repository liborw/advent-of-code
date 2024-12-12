use num::Num;
use crate::vector::Vec2;


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

    pub const DIRECTION_4: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];

    pub const DIRECTION_8: [Direction; 8] = [
        Direction::North,
        Direction::NorthEast,
        Direction::East,
        Direction::SouthEast,
        Direction::South,
        Direction::SouthWest,
        Direction::West,
        Direction::NorthWest,
    ];

    pub fn turn_90_left(&self) -> Self {
        use Direction::*;
        match *self {
            North       => West,
            NorthEast   => NorthWest,
            East        => North,
            SouthEast   => NorthEast,
            South       => East,
            SouthWest   => SouthEast,
            West        => South,
            NorthWest   => SouthWest,
        }
    }

    pub fn turn_90_right(&self) -> Self {
        use Direction::*;
        match *self {
            North       => East,
            NorthEast   => SouthEast,
            East        => South,
            SouthEast   => SouthWest,
            South       => West,
            SouthWest   => NorthWest,
            West        => North,
            NorthWest   => NorthEast,
        }
    }

    pub fn oposite(&self) -> Self {
        use Direction::*;
        match *self {
            North       => South,
            NorthEast   => SouthWest,
            East        => West,
            SouthEast   => NorthWest,
            South       => North,
            SouthWest   => NorthEast,
            West        => East,
            NorthWest   => SouthEast,
        }
    }
}

impl<T: Num + Copy> From<&Direction> for Vec2<T> {
    fn from(value: &Direction) -> Self {
        use Direction::*;
        match *value {
            North       => Vec2::new( T::zero(), T::zero() - T::one()),
            South       => Vec2::new( T::zero(), T::zero() + T::one()),
            West        => Vec2::new( T::zero() - T::one(), T::zero()),
            East        => Vec2::new( T::zero() + T::one(), T::zero()),
            NorthWest   => Vec2::new( T::zero() - T::one(), T::zero() - T::one()),
            NorthEast   => Vec2::new( T::zero() + T::one(), T::zero() - T::one()),
            SouthWest   => Vec2::new( T::zero() - T::one(), T::zero() + T::one()),
            SouthEast   => Vec2::new( T::zero() + T::one(), T::zero() + T::one()),
        }
    }
}


impl<T: Num + Copy> From<&Direction> for (T, T) {
    fn from(value: &Direction) -> Self {
        use Direction::*;
        match *value {
            North       => ( T::zero(), T::zero() - T::one()),
            South       => ( T::zero(), T::zero() + T::one()),
            West        => ( T::zero() - T::one(), T::zero()),
            East        => ( T::zero() + T::one(), T::zero()),
            NorthWest   => ( T::zero() - T::one(), T::zero() - T::one()),
            NorthEast   => ( T::zero() + T::one(), T::zero() - T::one()),
            SouthWest   => ( T::zero() - T::one(), T::zero() + T::one()),
            SouthEast   => ( T::zero() + T::one(), T::zero() + T::one()),
        }
    }
}


impl TryFrom<&char> for Direction {
    type Error = String;
    fn try_from(value: &char) -> Result<Self, Self::Error> {
        use Direction::*;
        match value {
            '^' | 'N' => Ok(North),
            '>' | 'E' => Ok(East),
            'v' | 'S' => Ok(South),
            '<' | 'W' => Ok(West),
            c   => Err(format!("Not a direction: {}", c)),
        }
    }
}
