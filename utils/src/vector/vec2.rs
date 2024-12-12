use num::Num;
use std::{fmt::{Debug, Display, Formatter}, ops::{Add, AddAssign, Div, DivAssign, Mul, Neg, Rem, RemAssign, Sub, SubAssign}};


#[derive(Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Copy)]
pub struct Vec2<T> {
    pub x: T,
    pub y: T
}


impl<T> Vec2<T> {
    pub fn new(x: T, y: T) -> Self {
        Self{x, y}
    }

    pub fn zero() -> Self
    where T: Num + Copy
    {
        Self{x: T::zero(), y: T::zero()}
    }

    pub fn advance_n(&self, v: impl Into<Vec2<T>>, n: T) -> Self
    where T: Num + Copy
    {
        *self + v.into() * n
    }

    pub fn advance(&self, v: impl Into<Vec2<T>>) -> Self
    where T: Num + Copy
    {
        self.advance_n(v, T::one())
    }
}


impl<T: Display> Display for Vec2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({}, {})", self.x, self.y)?;
        Ok(())
    }
}

impl<T: Debug> Debug for Vec2<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "({:?}, {:?})", self.x, self.y)?;
        Ok(())
    }
}

impl<T> From<(T, T)> for Vec2<T> {
    fn from(value: (T, T)) -> Self {
        Self{x: value.0, y: value.1}
    }
}

macro_rules! bin_op {
    ($trait:tt, $func:ident) => {
        impl<T: Num + Copy> $trait for Vec2<T> {
            type Output = Self;

            fn $func(self, other: Self) -> Self::Output {
                Vec2{x: self.x.$func(other.x), y: self.y.$func(other.y)}
            }
        }

        impl<T: Num + Copy> $trait<T> for Vec2<T> {
            type Output = Self;

            fn $func(self, other: T) -> Self::Output {
                Vec2{x: self.x.$func(other), y: self.y.$func(other)}
            }
        }
    };
}

bin_op!(Add, add);
bin_op!(Sub, sub);
bin_op!(Div, div);
bin_op!(Rem, rem);


macro_rules! assign_op {
    ($trait:tt, $func:ident, $op:ident) => {
        impl<T: Num + Copy> $trait for Vec2<T> {
            fn $func(&mut self, rhs: Self) {
                self.x  = self.x.$op(rhs.x);
                self.y  = self.y.$op(rhs.y);
            }
        }
    };
}

assign_op!(AddAssign, add_assign, add);
assign_op!(SubAssign, sub_assign, sub);
assign_op!(DivAssign, div_assign, div);
assign_op!(RemAssign, rem_assign, rem);


impl<T: Num + Copy> Neg for Vec2<T> {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Vec2{x: T::zero() - self.x, y: T::zero() - self.y}
    }
}

impl<T: Num + Copy> Mul<T> for Vec2<T> {
    type Output = Self;

    fn mul(self, rhs: T) -> Self::Output {
        Vec2{x: self.x * rhs, y: self.y * rhs}
    }
}
