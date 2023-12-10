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

}

impl From<(isize, isize)> for Pos {

    fn from(value: (isize, isize)) -> Self {
        Pos::new(value.0, value.1)
    }
}

impl Add for Pos {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        (self.x + rhs.x, self.y + rhs.y).into()
    }
}
