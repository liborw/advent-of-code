use std::collections::HashMap;

use itertools::Itertools;
use pathfinding::prelude::astar;
use utils::{direction::Direction, map::{Map, SparseMap, Vec2}, run_task, took};

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input));
    run_task!(|| part2(input));
}

fn parse(input: &str) -> SparseMap<char> {
    SparseMap::from_str(input, &|c| Some(c))
}

type State = (Vec2, u8);

fn successors(state: &State, map: &SparseMap<char>) -> Vec<(State, usize)> {
    Direction::DIRECTION_4.into_iter().filter_map(|d| {
        let next = state.0.advance(&d);

        let next_cheat = if state.1 > 0 {
            state.1 - 1
        } else {
            0
        };

        match map.get(&next) {
            Some('.') | Some('E') => Some(((next, next_cheat), 1)),
            Some('#') if state.1 > 1 => Some(((next, next_cheat), 1)),
            _ => None
        }
    }).collect()
}

fn to_char(v: u8) -> char {
    match v {
        2 => '1',
        1 => '2',
        0 => '0',
        _ => '.'

    }

}


fn part1(input: &str) -> usize {
    let map = parse(input);
    let start = map.find_all(|c| c == 'S').next().unwrap();
    let target = map.find_all(|c| c == 'E').next().unwrap();

    map.print('.');

    let (path, base_cost) = astar(
        &(start, 0),
        |s| successors(s, &map),
        |(p, _)| (target - *p).manhatan().unsigned_abs(),
        |(p, _)| *p == target).unwrap();

    let mut cheats = HashMap::new();
    for (i, start) in path.into_iter().enumerate() {
        let (path, cost) = astar(
            &(start.0, 2),
            |s| successors(s, &map),
            |(p, _)| (target - *p).manhatan().unsigned_abs(),
            |(p, _)| *p == target).unwrap();

        //println!("diff {}", base_cost - (i + cost));
        //
        let diff = base_cost - (i + cost);
        if diff > 0 {
            *cheats.entry(diff).or_insert(0) += 1;
            let mut map = map.clone();
            path.into_iter()
                .take(3)
                .for_each(|(p, v)| {
                    map.insert(p, to_char(v));
                });
            println!("{diff}");
            map.print('.');
        }
    }

    for k in cheats.keys().sorted() {
        println!("{k} -> {}", cheats.get(k).unwrap());
    }

    cheats.into_iter().filter_map(|(k, v)| (k > 100).then_some(v)).sum()
}


fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day20_part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 1);
    }

    // #[test]
    // fn day20_part1_final_test() {
    //     let input = include_str!("../input.txt");
    //     assert_eq!(part1(input), 1);
    // }

    #[test]
    fn day20_part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn day20_part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1);
    }
}
