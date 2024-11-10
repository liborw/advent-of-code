use std::collections::HashSet;

use common::{direction::Direction, map::{SparseMap, Map}, pos::Pos};
use took::took;

macro_rules! aoc_task {
    ($f:expr) => {
        let (took, result) = took($f);
        println!("{} took: {} result: {}", stringify!($f), took, result);
    };
}

fn main() {
    let input = include_str!("../input.txt");
    aoc_task!(|| part1(input));
    aoc_task!(|| part2(input));
}

#[derive(Debug)]
struct Command {
    dir: Direction,
    len: isize,
}

fn parse(input: &str) -> Vec<Command> {
    input.lines().map(|l| {
        let mut parts = l.split_whitespace();
        let dir = match parts.next().unwrap() {
            "L" => Direction::Left,
            "U" => Direction::Up,
            "D" => Direction::Down,
            "R" => Direction::Right,
             s  => panic!("Unknown command: {s}")
        };
        let len = parts.next().unwrap().parse().unwrap();
        Command{ dir, len}
    }).collect()
}

fn parse2(input: &str) -> Vec<Command> {
    input.lines().map(|l| {
        let code = l.split_whitespace().skip(2).next().unwrap();
        let len = isize::from_str_radix(&code[2..7], 16).unwrap();

        let dir = match i8::from_str_radix(&code[7..8], 16).unwrap() {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            s  => panic!("Unknown command: {s}")
        };
        Command{ dir, len}
    }).collect()
}

fn solve_poly(poly: &[Command]) -> isize {
    let border_point_count: isize = poly.iter().map(|c| c.len).sum();

    use Direction::*;
    let val = poly.iter().fold((0, 0), |(v, x), c| {
        match c.dir {
            Up => (v, x - c.len),
            Down => (v, x + c.len),
            Left => (v + (c.len * x), x),
            Right => (v - (c.len * x), x),
        }
    }).0;

    val + border_point_count / 2 + 1

}

fn part1(input: &str) -> isize {
    let plan = parse(input);
    solve_poly(&plan)
}

fn part2(input: &str) -> isize {
    let plan = parse2(input);
    println!("{plan:?}");
    solve_poly(&plan)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 62);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 40745);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 952408144115);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1);
    }
}




