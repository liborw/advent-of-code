use crate::map::Pos;




pub enum Direction {
    North,
    South,
    West,
    East,
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast
}

impl Direction {
    pub fn all() -> Vec<Self> {
        vec![
            Direction::North,
            Direction::NorthWest,
            Direction::West,
            Direction::SouthWest,
            Direction::South,
            Direction::SouthEast,
            Direction::East,
            Direction::NorthEast
        ]
    }
}

impl From<&Direction> for Pos {

    fn from(value: &Direction) -> Self {
        match value {
            Direction::North => Pos::new(0, 1),
            Direction::NorthWest => Pos::new(-1, 1),
            Direction::West => Pos::new(-1, 0),
            Direction::SouthWest => Pos::new(-1, -1),
            Direction::South => Pos::new(0,-1),
            Direction::SouthEast => Pos::new(1, -1),
            Direction::East => Pos::new(1, 0),
            Direction::NorthEast => Pos::new(1, 1)
        }
    }
}
