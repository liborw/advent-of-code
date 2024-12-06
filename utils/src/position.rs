use std::{fmt::Display, ops::{Add, Mul, Neg, Sub}};

use num::{One, Zero};

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
pub struct Position<T> {
    pub row: T,
    pub col: T
}


impl<T> From<(T, T)> for Position<T> {
    fn from(value: (T, T)) -> Self {
        Position{row: value.0, col: value.1}
    }
}

impl<T: Display> Display for Position<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.row, self.col)
    }
}

impl<T: Add<Output = T>> Add<Position<T>> for Position<T> {
    type Output = Position<T>;
    fn add(self, rhs: Position<T>) -> Self::Output {
        Position{row:  self.row + rhs.row, col: self.col + rhs.col}
    }
}

impl<T: Mul<Output = T> + Copy> Mul<T> for Position<T> {
    type Output = Position<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Position{row: self.row * rhs, col: self.col * rhs}
    }
}

impl<T> Position<T>
where
    T:  Add<Output = T> + Sub<Output = T> + Mul<Output = T> + Copy
{

    pub fn new(row: T, col: T) -> Self {
        Self{row, col}
    }

    pub fn next(&self, dir: impl Into<Position<T>>, steps: T) -> Self {
        self.clone() + dir.into() * steps
    }
}

impl<T: One + Zero + Neg<Output=T>> From<Direction> for Position<T> {
    fn from(value: Direction) -> Self {
        use Direction::*;
        match value {
            Up    => Position{row: -T::one(),  col:  T::zero()},
            Right => Position{row:  T::zero(), col:  T::one()},
            Down  => Position{row:  T::one(),  col:  T::zero()},
            Left  => Position{row:  T::zero(), col: -T::one()},
        }
    }
}


#[derive(Debug, PartialEq, Eq, Clone, Hash, Copy)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Direction {

    pub fn all_c() -> impl Iterator<Item=Direction> {
        use Direction::*;
        vec![Up, Right, Down, Left].into_iter()
    }

    pub fn all_ac() -> impl Iterator<Item=Direction> {
        use Direction::*;
        vec![Up, Left, Down, Right].into_iter()
    }

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

    #[test]
    fn next_test() {
        let p = Position::new(0, 0);
        assert_eq!(p.next(Direction::Down, 2), (2, 0).into());
        assert_eq!(p.next(Direction::Up, 2), (-2, 0).into());
        assert_eq!(p.next(Direction::Left, -2), (0, 2).into());
        assert_eq!(p.next(Direction::Right, 2), (0, 2).into());
    }

}
