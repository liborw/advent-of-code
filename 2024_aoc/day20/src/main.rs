use std::{collections::HashMap};

use itertools::Itertools;
use pathfinding::{num_traits::{CheckedNeg, CheckedSub}, prelude::{astar, build_path, dijkstra_all}};
use utils::{direction::Direction, map::{Map, SparseMap, Vec2}, run_task, took};

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input));
    run_task!(|| part2(input));
}

fn parse(input: &str) -> SparseMap<char> {
    SparseMap::from_str(input, &|c| Some(c))
}

fn successors(state: &Vec2, map: &SparseMap<char>) -> Vec<(Vec2, usize)> {
    Direction::DIRECTION_4.into_iter().filter_map(|d| {
        let next = state.advance(&d);

        match map.get(&next) {
            Some('.') | Some('E') | Some('C') | Some('S') => Some((next, 1)),
            _ => None
        }
    }).collect()
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    map.print('.');
    let cheats = solve(map, 2);

    for k in cheats.keys().sorted() {
        println!("{k} -> {}", cheats.get(k).unwrap());
    }

    cheats.into_iter().filter_map(|(k, v)| (k >= 100).then_some(v)).sum()
}

fn all_in_range(pos: Vec2, radius: isize) -> Vec<Vec2> {
    let mut out = vec![];
    for x in -radius..=radius {
        for y in -radius..=radius {
            let test = pos + Vec2::new(x, y);
            if (test - pos).manhatan() <= radius {
                out.push(test);
            }
        }
    }
    out
}


fn solve(mut map: SparseMap<char>, t_cheat: usize) -> HashMap<usize, usize> {
    let start = map.find_all(|c| c == 'S').next().unwrap();
    let target = map.find_all(|c| c == 'E').next().unwrap();

    let costs = dijkstra_all(&target, |n| successors(n, &map));
    let base_cost = costs.get(&start).unwrap().1;

    let mut cheats = HashMap::new();
    let mut path = build_path(&start, &costs);
    path.reverse();

    for (i, start) in path.into_iter().enumerate() {

        for n in all_in_range(start, t_cheat as isize) {

            if let Some((_, c)) = costs.get(&n) {
                let new_cost = i + (start - n).manhatan() as usize + *c;

                if let Some(diff) = base_cost.checked_sub(new_cost) {
                    if diff > 0 {
                        *cheats.entry(diff).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    cheats
}


fn part2(input: &str) -> usize {
    let map = parse(input);
    map.print('.');
    let cheats = solve(map, 20);

    for k in cheats.keys().sorted() {
        println!("{k} -> {}", cheats.get(k).unwrap());
    }

    cheats.into_iter().filter_map(|(k, v)| (k >= 100).then_some(v)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day20_part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 1);
    }

    //#[test]
    //fn day20_part1_final_test() {
    //    let input = include_str!("../input.txt");
    //    assert_eq!(part1(input), 1296);
    //}

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
