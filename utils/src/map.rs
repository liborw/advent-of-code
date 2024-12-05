use std::{collections::HashMap, fmt::Display, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32
}


impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Self{x, y}
    }

    pub fn zero() -> Self {
        Self{x: 0, y: 0}
    }

    pub fn r#move(&self, dir: &Direction, steps: i32) -> Self {
        use Direction::*;
        match dir {
            North => *self + (0, steps).into(),
            NorthWest => *self + (-steps, steps).into(),
            West => *self + (-steps, 0).into(),
            SouthWest => *self + (-steps, -steps).into(),
            South => *self + (0,-steps).into(),
            SouthEast => *self + (steps, -steps).into(),
            East => *self + (steps, 0).into(),
            NorthEast => *self + (steps, steps).into()
        }
    }
}

impl From<(i32, i32)> for Pos {
    fn from(value: (i32, i32)) -> Self {
        Pos::new(value.0, value.1)
    }
}

impl Add for Pos {
    type Output = Pos;

    fn add(self, rhs: Self) -> Self::Output {
        Self{x: self.x + rhs.x, y: self.y + rhs.y}
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Bounds {
    pub x_min: i32,
    pub y_min: i32,
    pub x_max: i32,
    pub y_max: i32
}

impl Bounds {

    pub fn new(x_min: i32, x_max: i32, y_min: i32, y_max: i32) -> Self {
        Bounds{x_min, x_max, y_min, y_max}
    }

    pub fn zero() -> Self {
        Bounds{x_min: 0, y_min: 0, x_max: 0, y_max: 0}
    }
}

pub type SparseMap<T> = HashMap<Pos, T>;

pub trait Map<T> {
    fn bounds(&self) -> Bounds;
    fn print(&self, bg: T);
    fn print_with_bounds(&self, bg: T, bounds: &Bounds);
    fn from_str(input: &str, elem_fn: &dyn Fn(char) -> Option<T>) -> SparseMap<T>;
    fn find_all(&self, predicate: &dyn Fn(&T) -> bool) -> impl Iterator<Item=Pos>;
}

impl<T: Display> Map<T> for SparseMap<T> {

    fn find_all(&self, predicate: &dyn Fn(&T) -> bool) -> impl Iterator<Item=Pos> {
        self.iter().filter_map(|(&k, v)| predicate(v).then_some(k))
    }

    fn bounds(&self) -> Bounds {
        let mut b = Bounds::zero();

        self.keys().for_each(|p| {
            if p.x > b.x_max { b.x_max = p.x}
            if p.x < b.x_min { b.x_min = p.x}
            if p.y > b.y_max { b.y_max = p.y}
            if p.y < b.y_min { b.y_min = p.y}
        });

        b
    }

    fn print(&self, bg: T) {
        let b = self.bounds();
        for r in b.x_min..=b.x_max  {
            for c in b.y_min..=b.y_max {
                print!("{}", self.get(&Pos::new(r, c)).unwrap_or(&bg));
            }
            println!();
        }
    }

    fn print_with_bounds(&self, bg: T, bounds: &Bounds) {
        for r in bounds.x_min..=bounds.x_max  {
            for c in bounds.y_min..=bounds.y_max {
                print!("{}", self.get(&Pos::new(r, c)).unwrap_or(&bg));
            }
            println!();
        }
    }

    fn from_str(input: &str, elem_fn: &dyn Fn(char) -> Option<T>) -> SparseMap<T> {
        input.lines().enumerate().flat_map(|(row, l)| {
            l.chars().enumerate().filter_map(move |(col, c)| {
                elem_fn(c).map(|v| ((row as i32, col as i32).into(), v))
            })
        }).collect()
    }
}
