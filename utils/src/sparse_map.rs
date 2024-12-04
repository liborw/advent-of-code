use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Pos {
    pub x: i32,
    pub y: i32
}

impl Pos {
    pub fn new(x: i32, y: i32) -> Self {
        Pos{x, y}
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

    pub fn offset(&mut self, offset: i32) {
        self.x_max += offset;
        self.y_max += offset;
        self.x_min -= offset;
        self.y_min -= offset;
    }

}

pub trait Map<T> {
    fn bounds(&self) -> Bounds;
    fn print_with_bounds(&self, bg: T, bounds: &Bounds);

    fn print(&self, bg: T) {
        self.print_with_bounds(bg, &self.bounds());
    }
}

pub fn n_digits(n: i32) -> usize {
    n.to_string()
        .chars()
        .count()
}

pub fn digits(n: i32) -> Vec<char> {
    n.to_string()
        .chars()
        .collect()
}

pub fn x_ticks(from: i32, to: i32, padding: usize) -> String {

    let numbers: Vec<String> = (from..=to).map(|v| v.abs().to_string()).collect();
    let n_digits = numbers.iter().map(|v| v.len()).max().unwrap();

    let mut output = String::new();

    for l in 0..n_digits {
        // add padding
        output.push_str(&" ".repeat(padding));

        let i = n_digits - l - 1;
        for number in numbers.iter() {
            output.push( number.chars().rev().nth(i).unwrap_or(' '));
        }
        output.push('\n');
    }
    output
}

pub fn y_ticks(from: i32, to:i32) -> Vec<String> {

    let mut numbers: Vec<String> = (from..=to).map(|v| v.abs().to_string()).collect();
    let n_digits = numbers.iter().map(|v| v.len()).max().unwrap();

    // padding
    for number in numbers.iter_mut() {
        let pad = n_digits - number.len();
        *number = " ".repeat(pad) + number;
    }

    numbers
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

    fn print_with_bounds(&self, bg: T, bounds: &Bounds) {
        let mut bounds = bounds.clone();
        bounds.offset(10);

        let y_ticks = y_ticks(bounds.y_min, bounds.y_max);
        print!("{}", x_ticks(bounds.x_min, bounds.x_max, y_ticks[0].len()));

        for r in bounds.y_min..=bounds.y_max  {
            print!("{}", y_ticks[(r - bounds.y_min) as usize]);
            for c in bounds.x_min..=bounds.x_max {
                print!("{}", self.get(&Pos::new(c, r)).unwrap_or(&bg));
            }
            println!();
        }
    }
}


#[cfg(test)]
mod test {
    use core::panic;

    use super::*;

    #[test]
    fn digits_test() {
        assert_eq!(digits(1), vec!['1']);
        assert_eq!(digits(21), vec!['2', '1']);
    }

    #[test]
    fn n_digits_test() {
        assert_eq!(n_digits(1), 1);
        assert_eq!(n_digits(92513), 5);
    }

    #[test]
    fn x_ticks_test() {
        assert_eq!(x_ticks(0, 3, 0), "0123\n".to_string());
        assert_eq!(x_ticks(0, 3, 2), "  0123\n".to_string());
        assert_eq!(x_ticks(-1, 1, 0), "101\n".to_string());
        assert_eq!(x_ticks(8, 11, 0), "  11\n8901\n".to_string());
    }

    #[test]
    fn print_test() {
        let mut map = HashMap::new();
        map.insert(Pos::new(0, 0), 'o');
        map.insert(Pos::new(2, -2), 'o');
        map.insert(Pos::new(12, 12), 'x');

        map.print('.');
        panic!();
    }


}
