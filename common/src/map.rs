
use std::collections::HashMap;
use std::fmt::Display;
use crate::pos::Pos;


pub struct BoundingBox {
    pub x_min: isize,
    pub x_max: isize,
    pub y_min: isize,
    pub y_max: isize
}

impl BoundingBox {
    pub fn new(x_min: isize, x_max: isize, y_min: isize, y_max: isize) -> Self {
        BoundingBox{x_min, x_max, y_min, y_max}
    }

    pub fn zero() -> Self {
        BoundingBox::new(isize::MAX, isize::MIN, isize::MAX, isize::MIN)
    }

    pub fn xsize(&self) -> usize {
        (self.x_max + self.x_min).abs() as usize
    }

    pub fn ysize(&self) -> usize {
        (self.y_max + self.y_min).abs() as usize
    }
}

pub type SparseMap<T> = HashMap<Pos, T>;

pub trait Map<T> {
    fn bounding_box(&self) -> BoundingBox;
    fn dump(&self, bg: T) -> ();
    fn from_str(input: &str, elem_fn: &dyn Fn(char) -> Option<T>) -> SparseMap<T>;
}

impl<T: Display> Map<T> for SparseMap<T> {

    fn bounding_box(&self) -> BoundingBox {
        let mut bb = BoundingBox::zero();

        self.keys().for_each(|p| {
            if p.x > bb.x_max { bb.x_max = p.x}
            if p.x < bb.x_min { bb.x_min = p.x}
            if p.y > bb.y_max { bb.y_max = p.y}
            if p.y < bb.y_min { bb.y_min = p.y}

        });

        bb
    }

    fn dump(&self, bg: T) -> () {
        let bb = self.bounding_box();
        for r in bb.x_min..=bb.x_max  {
            for c in bb.y_min..=bb.y_max {
                print!("{}", self.get(&Pos::new(r, c)).unwrap_or(&bg));
            }
            println!();
        }
    }

    fn from_str(input: &str, elem_fn: &dyn Fn(char) -> Option<T>) -> SparseMap<T> {
        input.lines().enumerate().map(|(row, l)| {
            l.chars().enumerate().filter_map(move |(col, c)| {
                elem_fn(c).map(|v| ((row, col).into(), v))
            })
        }).flatten().collect()
    }
}

