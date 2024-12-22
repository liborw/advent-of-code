use std::{collections::HashMap, fmt::Display};

use crate::vector::{self, Rect};


pub type Vec2 = vector::Vec2<isize>;
pub type SparseMap<T> = HashMap<Vec2, T>;

pub trait Map<T> {
    fn copy_map(&self, elem_f: impl Fn(&T) -> Option<T>) -> Self;
    fn bounds(&self) -> Rect<isize>;
    fn print_with_bounds(&self, bg: char, bounds: &Rect<isize>);
    fn map_print_with_bounds(&self, bg: char, bounds: &Rect<isize>, elem_f: impl Fn(&T) -> char);
    fn from_str(input: &str, elem_fn: &dyn Fn(char) -> Option<T>) -> SparseMap<T>;
    fn find_all(&self, predicate: impl Fn(T) -> bool) -> impl Iterator<Item=Vec2>;

    fn find_first(&self, predicate: impl Fn(T) -> bool) -> Option<Vec2> {
        self.find_all(predicate).next()
    }

    fn print(&self, bg: char) {
        let b = self.bounds();
        self.print_with_bounds(bg, &b);
    }
}


impl<T: Display + Copy> Map<T> for SparseMap<T> {
    fn copy_map(&self, elem_f: impl Fn(&T) -> Option<T>) -> Self {
        let mut map = SparseMap::new();

        self.iter().for_each(|(p, v)| {
            if let Some(v) = elem_f(v) {
                map.insert(*p, v);
            }
        });

        map
    }

    fn find_all(&self, predicate: impl Fn(T) -> bool) -> impl Iterator<Item=Vec2> {

        self.iter().filter_map(move |(&k, v)| predicate(*v).then_some(k))
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

    fn print_with_bounds(&self, bg: char, bounds: &Rect<isize>) {

        for r in bounds.min.y..=bounds.max.y  {
            for c in bounds.min.x..=bounds.max.x {
                if let Some(v) = self.get(&(c, r).into()) {
                    print!("{}", v);
                } else {
                    print!("{bg}");
                }
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

    fn map_print_with_bounds(&self, bg: char, bounds: &Rect<isize>, elem_f: impl Fn(&T) -> char) {
        for r in bounds.min.y..=bounds.max.y  {
            for c in bounds.min.x..=bounds.max.x {
                if let Some(v) = self.get(&(c, r).into()) {
                    print!("{}", elem_f(v));
                } else {
                    print!("{bg}");
                }
            }
            println!();
        }

    }
}
