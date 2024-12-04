use std::{collections::HashMap, fmt::Display, ops::Add};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32
}


impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos{x, y}
    }

    pub fn r#move(&self, dir: &Direction, steps: i32) -> Self {
        use Direction::*;
        match dir {
            North => Pos::new(0, steps),
            NorthWest => Pos::new(-steps, steps),
            West => Pos::new(-steps, 0),
            SouthWest => Pos::new(-steps, -steps),
            South => Pos::new(0,-steps),
            SouthEast => Pos::new(steps, -steps),
            East => Pos::new(steps, 0),
            NorthEast => Pos::new(steps, steps)
        }
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

pub trait Map<T> {
    fn bounds(&self) -> Bounds;
    fn print(&self, bg: T);
    fn print_with_bounds(&self, bg: T, bounds: &Bounds);
}


impl<T: Display> Map<T> for HashMap<Pos, T> {

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
}
