
use std::collections::HashSet;

use pathfinding::prelude::{astar, astar_bag};
use utils::{direction::Direction, map::{Map, SparseMap, Vec2}, run_task, took};

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input));
    run_task!(|| part2(input));
}

fn parse(input: &str) -> SparseMap<char> {
    SparseMap::from_str(input, &|c| Some(c))
}

type State = (Vec2, Direction);

fn successors(state: &State, map: &SparseMap<char>) -> Vec<(State, usize)> {


    let mut out = vec![
        ((state.0, state.1.turn_90_left()), 1000),
        ((state.0, state.1.turn_90_right()), 1000),
    ];

    let adv = state.0.advance(&state.1);
    match map.get(&adv) {
        None => unreachable!("Outside maze: {adv}"),
        Some('#') => (),
        Some(_) => out.push(((adv, state.1), 1)),
    }

    out
}


// 30 m
fn part1(input: &str) -> usize {
    let map = parse(input);
    let start = map.find_all(|c| c=='S' ).next().unwrap();
    let target = map.find_all(|c| c=='E' ).next().unwrap();

    let (_, cost) = astar(
        &(start, Direction::East),
        |s| successors(s, &map),
        |(p, _)| (target - *p).manhatan() as usize,
        |(p, _)| map.get(p) == Some(&'E')).unwrap();
    cost
}

// 10m
fn part2(input: &str) -> usize {
    let map = parse(input);
    let start = map.find_all(|c| c=='S' ).next().unwrap();
    let target = map.find_all(|c| c=='E' ).next().unwrap();

    let (solutions, _) = astar_bag(
        &(start, Direction::East),
        |s| successors(s, &map),
        |(p, _)| (target - *p).manhatan() as usize,
        |(p, _)| map.get(p) == Some(&'E')).unwrap();


    let mut set = HashSet::new();
    set.insert(start);
    set.insert(target);
    for s in solutions {
        for n in s {
            set.insert(n.0);
        }
    }

    set.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day16_part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 7036);
    }

    #[test]
    fn day16_part1_test2() {
        let input = include_str!("../input_test2.txt");
        assert_eq!(part1(input), 11048);
    }

    #[test]
    fn day16_part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 114476);
    }

    #[test]
    fn day16_part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 45);
    }

    #[test]
    fn day16_part2_test2() {
        let input = include_str!("../input_test2.txt");
        assert_eq!(part2(input), 64);
    }

    #[test]
    fn day16_part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 508);
    }
}
