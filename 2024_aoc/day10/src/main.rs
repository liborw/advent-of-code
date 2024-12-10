use std::{collections::{HashSet, VecDeque}, hash::Hash};
use utils::{direction::{cardinal::Direction, AdvanceInDirection}, map::{Map, SparseMap, Vec2}, run_task, took};

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input));
    run_task!(|| part2(input));
}

pub fn bfs_cnt<N, F, G, IN>(start: N, expand: F, predicate: G) -> usize
where
    N: Copy + Eq + Hash,
    F: Fn(&N) -> IN,
    IN: IntoIterator<Item = N>,
    G: Fn(&N) -> bool,
{

    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    let mut cnt = 0;

    visited.insert(start);
    queue.push_back(start);

    while let Some(n) = queue.pop_front() {

        if predicate(&n) {
            cnt += 1;
        }

        for next in expand(&n).into_iter() {
            if visited.insert(next) {
                queue.push_back(next);
            }
        }
    }
    cnt
}


pub fn dfs_cnt<N, F, G, IN>(start: N, expand: F, predicate: G) -> usize
where
    N: Copy + Eq + Hash,
    F: Fn(&N) -> IN,
    IN: IntoIterator<Item = N>,
    G: Fn(&N) -> bool,
{
    let mut stack = Vec::new();
    let mut cnt = 0;

    stack.push(start);

    while let Some(n) = stack.pop() {

        if predicate(&n) {
            cnt += 1;
        }

        for next in expand(&n).into_iter() {
            stack.push(next);
        }
    }
    cnt
}


fn parse(input: &str) -> SparseMap<u8> {
    SparseMap::from_str(input, &|c| c.to_digit(10).map(|d| d as u8))
}

fn expand(p: &Vec2, map: &SparseMap<u8>) -> Vec<Vec2> {
    let cur = map.get(p).unwrap();

    Direction::ALL.into_iter().filter_map(|d| {
        let next = p.advance(&d);
        map.get(&next).is_some_and(|v| *v == *cur + 1).then_some(next)
    }).collect()
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    let starts = map.find_all(&|d| d == &0);

    starts.into_iter().map(|s| {
        bfs_cnt(s, |p| expand(p, &map), |v| map.get(v).is_some_and(|d| d == &9))
    }).sum()

}

fn part2(input: &str) -> usize {
    let map = parse(input);
    let starts = map.find_all(&|d| d == &0);

    starts.into_iter().map(|s| {
        dfs_cnt(s, |p| expand(p, &map), |v| map.get(v).is_some_and(|d| d == &9))
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day10_part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 36);
    }

    #[test]
    fn day10_part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 552);
    }

    #[test]
    fn day10_part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 81);
    }

    #[test]
    fn day10_part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1225);
    }
}
