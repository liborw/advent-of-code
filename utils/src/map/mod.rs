use std::{collections::HashMap, fmt::Display};

use crate::vector::{self, Rect};


pub type Vec2 = vector::Vec2<isize>;
pub type SparseMap<T> = HashMap<Vec2, T>;

pub trait Map<T> {
    fn copy_map(&self, elem_f: impl Fn(&T) -> Option<T>) -> Self;
    fn bounds(&self) -> Rect<isize>;
    fn print(&self, bg: T);
    fn print_with_bounds(&self, bg: T, bounds: &Rect<isize>);
    fn from_str(input: &str, elem_fn: &dyn Fn(char) -> Option<T>) -> SparseMap<T>;
    fn find_all(&self, predicate: &dyn Fn(&T) -> bool) -> impl Iterator<Item=Vec2>;
}


impl<T: Display> Map<T> for SparseMap<T> {
    fn copy_map(&self, elem_f: impl Fn(&T) -> Option<T>) -> Self {
        let mut map = SparseMap::new();

        self.iter().for_each(|(p, v)| {
            if let Some(v) = elem_f(v) {
                map.insert(*p, v);
            }
        });

        map
    }

    fn find_all(&self, predicate: &dyn Fn(&T) -> bool) -> impl Iterator<Item=Vec2> {
        self.iter().filter_map(|(&k, v)| predicate(v).then_some(k))
    }

    fn bounds(&self) -> Rect<isize> {
        let mut b = Rect::zero();

        self.keys().for_each(|p| {
            if p.x > b.max.x { b.max.x = p.x}
            if p.x < b.min.x { b.min.x = p.x}
            if p.y > b.max.y { b.max.y = p.y}
            if p.y < b.min.y { b.min.y = p.y}
        });

        b
    }

    fn print(&self, bg: T) {
        let b = self.bounds();
        self.print_with_bounds(bg, &b);
    }

    fn print_with_bounds(&self, bg: T, bounds: &Rect<isize>) {

        for r in bounds.min.x..=bounds.max.x  {
            for c in bounds.min.y..=bounds.max.y {
                print!("{}", self.get(&Vec2::new(c, r)).unwrap_or(&bg));
            }
            println!();
        }
    }

    fn from_str(input: &str, elem_fn: &dyn Fn(char) -> Option<T>) -> SparseMap<T> {
        input.lines().enumerate().flat_map(|(row, l)| {
            l.chars().enumerate().filter_map(move |(col, c)| {
                elem_fn(c).map(|v| ((col as isize, row as isize).into(), v))
            })
        }).collect()
    }
}
