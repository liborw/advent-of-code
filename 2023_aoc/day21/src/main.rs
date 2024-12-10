use std::collections::HashSet;

use common::{map::SparseMap, pos::Pos};
use took::took;

macro_rules! aoc_task {
    ($f:expr) => {
        let (took, result) = took($f);
        println!("{} took: {} result: {}", stringify!($f), took, result);
    };
}

fn main() {
    let input = include_str!("../input.txt");
    aoc_task!(|| part1(input, 64));
    aoc_task!(|| part2(input));
}

fn parse_grid<T>(input: &str, elem_fn: &dyn Fn(char) -> Option<T>) -> SparseMap<T> {
    input.lines().enumerate().map(|(row, l)| {
        l.chars().enumerate().filter_map(move |(col, c)| {
            elem_fn(c).map(|v| ((row, col).into(), v))
        })
    }).flatten().collect()
}

fn parse(input: &str) -> SparseMap<char> {
    parse_grid(input, &|c| {
        if c != '.' {
            Some(c)
        } else {
            None
        }
    })
}

fn part1(input: &str, steps: usize) -> usize {
    let map = parse(input);
    let mut positions: HashSet<Pos> = HashSet::new();
    positions.insert(map.iter().find(|(_, &c)| c == 'S').unwrap().0.clone());

    for _ in 0..steps {
        positions = positions.into_iter()
            .map(|p| {
                p.neighbors4().into_iter().filter(|p| !map.get(&p).is_some_and(|&c| c == '#'))
            }).flatten().collect()
    }
    positions.len()
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
        assert_eq!(part1(input, 6), 16);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input, 64), 3768);
    }

    // #[test]
    // fn part2_test() {
    //     let input = include_str!("../input_test.txt");
    //     assert_eq!(part2(input), 1);
    // }

    // #[test]
    // fn part2_final_test() {
    //     let input = include_str!("../input.txt");
    //     assert_eq!(part2(input), 1);
    // }
}
