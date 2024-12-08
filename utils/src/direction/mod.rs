use num::{Num, Signed};
use crate::vector::Vec2;

pub mod cardinal;
pub mod ordinal;



pub trait AdvanceInDirection<T>
    where Self: Sized {
    type Unit: Num;
    fn advance(&self, dir: T) -> Self;
    fn advance_n(&self, dir: T, steps: Self::Unit) -> Self;
}


impl<T: Num + Copy> AdvanceInDirection<&cardinal::Direction> for Vec2<T> {
    type Unit = T;

    fn advance(&self, dir: &cardinal::Direction) -> Self {
        self.advance_n(dir, T::one())
    }

    fn advance_n(&self, dir: &cardinal::Direction, steps: Self::Unit) -> Self {
        use cardinal::Direction::*;
        match &dir {
            Up    => Vec2::new( self.x, self.y - steps),
            Down  => Vec2::new( self.x, self.y + steps),
            Left  => Vec2::new( self.x - steps, self.y),
            Right => Vec2::new( self.x + steps, self.y),
        }
    }
}

impl<T: Num + Copy> AdvanceInDirection<&ordinal::Direction> for Vec2<T> {
    type Unit = T;

    fn advance(&self, dir: &ordinal::Direction) -> Self {
        self.advance_n(dir, T::one())
    }

    fn advance_n(&self, dir: &ordinal::Direction, steps: Self::Unit) -> Self {
        use ordinal::Direction::*;
        match dir {
            North => Vec2::new( self.x, self.y - steps),
            South => Vec2::new( self.x, self.y + steps),
            West => Vec2::new( self.x - steps, self.y),
            East => Vec2::new( self.x + steps, self.y),
            NorthWest => Vec2::new( self.x - steps, self.y - steps),
            NorthEast => Vec2::new( self.x + steps, self.y - steps),
            SouthWest => Vec2::new( self.x - steps, self.y + steps),
            SouthEast => Vec2::new( self.x + steps, self.y + steps),
        }
    }
}

impl<T: Num + Copy> From<&cardinal::Direction> for (T, T) {
    fn from(value: &cardinal::Direction) -> Self {
        use cardinal::Direction::*;
        match &value {
            Up    => (T::zero(), T::zero() - T::one()),
            Down  => (T::zero(), T::one()),
            Left  => (T::zero() - T::one(), T::zero()),
            Right => (T::one(),  T::zero()),
        }
    }
}

impl<T: Num + Signed + Copy> From<&cardinal::Direction> for Vec2<T> {

    fn from(value: &cardinal::Direction) -> Self {
        use cardinal::Direction::*;
        match &value {
            Up    => Vec2::new(T::zero(), -T::one()),
            Down  => Vec2::new(T::zero(), T::one()),
            Left  => Vec2::new(-T::one(), T::zero()),
            Right => Vec2::new(T::one(),  T::zero()),
        }

    }

}


