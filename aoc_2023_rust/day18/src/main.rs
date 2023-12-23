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

fn part1(input: &str) -> isize {
    let plan = parse(input);
    let mut edges: Vec<(Pos, Direction, isize)> = Vec::new();
    let mut pos = Pos::new(0,0);

    plan.iter().for_each(|c| {
        edges.push((pos, c.dir, c.len));
        pos = (0..c.len).fold(pos, |p, _| c.dir.move_pos(&p));
    });

    let mut map: SparseMap<char> = SparseMap::new();
    edges.iter().for_each(|&(p, _, _)| {
        map.insert(p, '#');
    });
    map.dump('.');

    let xmax = edges.iter().map(|(p, _, _)| p.x).max().unwrap();
    println!("Xmin: {xmax}");
    println!("Edges: {edges:?}");

    edges.iter().fold(0,|v, &(p, d, l)| {

        match d {
            Direction::Left => {
                let delta = -(xmax - p.x) * l;
                println!("Left  {v} + {delta} = {} ", v + delta);
                v + delta
            },
            Direction::Right => {
                let delta = (xmax - p.x + 1) * (l + 1);
                println!("Right  {v} + {delta} = {} ", v + delta);
                v + delta
            }
            _ => v
        }
    })
}

fn part2(input: &str) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 62);
    }

    // #[test]
    // fn part1_final_test() {
    //     let input = include_str!("../input.txt");
    //     assert_eq!(part1(input), 40745);
    // }

    //#[test]
    //fn part2_test() {
    //    let input = include_str!("../input_test.txt");
    //    assert_eq!(part2(input), 1);
    //}

    //#[test]
    //fn part2_final_test() {
    //    let input = include_str!("../input.txt");
    //    assert_eq!(part2(input), 1);
    //}
}
