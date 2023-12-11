use std::ops::Add;

#[derive(Debug, Eq, Hash, PartialEq, Clone, Copy)]
pub struct Pos {
    pub x: isize,
    pub y: isize
}

impl Pos {
    pub fn new(x: isize, y: isize) -> Self {
        Self { x, y }
    }

    pub fn neighbors8(&self) -> Vec<Pos> {
        vec![
            *self + ( 1,  0).into(),
            *self + ( 1,  1).into(),
            *self + ( 0,  1).into(),
            *self + (-1,  1).into(),
            *self + (-1,  0).into(),
            *self + (-1, -1).into(),
            *self + ( 0, -1).into(),
            *self + ( 1, -1).into(),
        ]
    }

    pub fn neighbors4(&self) -> Vec<Pos> {
        vec![
            *self + ( 1,  0).into(),
            *self + ( 0,  1).into(),
            *self + (-1,  0).into(),
            *self + ( 0, -1).into(),
        ]
    }

    pub fn dist_manhatan(&self, other: Self) -> usize {
        ((self.x - other.x).abs() + (self.y - other.y).abs()) as usize
    }

}

impl From<(isize, isize)> for Pos {
    fn from(value: (isize, isize)) -> Self {
        Pos::new(value.0, value.1)
    }
}

impl From<(usize, usize)> for Pos {
    fn from(value: (usize, usize)) -> Self {
        Pos::new(value.0 as isize, value.1 as isize)
    }
}

impl From<(i32, i32)> for Pos {
    fn from(value: (i32, i32)) -> Self {
        Pos::new(value.0 as isize, value.1 as isize)
    }
}

impl From<(u32, u32)> for Pos {
    fn from(value: (u32, u32)) -> Self {
        Pos::new(value.0 as isize, value.1 as isize)
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self.x + rhs.x, self.y + rhs.y).into()
    }
}
