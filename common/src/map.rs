
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Hash, Eq, PartialEq)]
pub enum Adjacency {
    Four,
    Eigth,
    Diag
}

#[derive(Debug, Hash, Eq, PartialEq)]
pub struct Position {
    pub row: isize,
    pub col: isize
}


impl Position {
    pub fn new(row: isize, col: isize) -> Self {
        Position{row, col}
    }

    pub fn neightbours(&self, adjacency: Adjacency) -> Vec<Position> {
        match adjacency {
            Adjacency::Four => {
                vec![
                    Position::new(self.row - 1, self.col),
                    Position::new(self.row, self.col + 1),
                    Position::new(self.row + 1, self.col),
                    Position::new(self.row, self.col - 1),
                ]
            },
            Adjacency::Eigth => {
                vec![
                    Position::new(self.row - 1, self.col),
                    Position::new(self.row - 1, self.col + 1),
                    Position::new(self.row, self.col + 1),
                    Position::new(self.row + 1, self.col + 1),
                    Position::new(self.row + 1, self.col),
                    Position::new(self.row + 1, self.col - 1),
                    Position::new(self.row, self.col - 1),
                    Position::new(self.row - 1, self.col - 1)
                ]
            },
            Adjacency::Diag => {
                vec![
                    Position::new(self.row - 1, self.col + 1),
                    Position::new(self.row + 1, self.col + 1),
                    Position::new(self.row + 1, self.col - 1),
                    Position::new(self.row - 1, self.col - 1)
                ]
            }
        }
    }
}

pub struct BoundingBox {
    pub row_min: isize,
    pub row_max: isize,
    pub col_min: isize,
    pub col_max: isize
}

impl BoundingBox {
    pub fn new(row_min: isize, row_max: isize, col_min: isize, col_max: isize) -> Self {
        BoundingBox{row_min, row_max, col_min, col_max}
    }

    pub fn zero() -> Self {
        BoundingBox::new(isize::MAX, isize::MIN, isize::MAX, isize::MIN)
    }
}

pub type SparseMap<T> = HashMap<Position, T>;

pub trait Map<T> {
    fn bounding_box(&self) -> BoundingBox;
    fn dump(&self, bg: T) -> ();
}

impl<T: Display> Map<T> for SparseMap<T> {

    fn bounding_box(&self) -> BoundingBox {
        let mut bb = BoundingBox::zero();

        self.keys().for_each(|p| {
            if p.row > bb.row_max { bb.row_max = p.row }
            if p.row < bb.row_min { bb.row_min = p.row }
            if p.col > bb.col_max { bb.col_max = p.col}
            if p.col < bb.col_min { bb.col_min = p.col}

        });

        bb
    }

    fn dump(&self, bg: T) -> () {
        let bb = self.bounding_box();
        for r in bb.row_min..=bb.row_max  {
            for c in bb.col_min..=bb.col_max {
                print!("{}", self.get(&Position::new(r, c)).unwrap_or(&bg));
            }
            println!();
        }
    }
}
