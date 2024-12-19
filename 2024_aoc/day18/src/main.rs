use itertools::Itertools;
use pathfinding::prelude::astar;
use utils::{direction::Direction, map::{SparseMap, Vec2}, run_task, took, vector::Rect};

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input, (70, 70), 1024));
    run_task!(|| part2(input, (70, 70)));
}

fn parse(input: &str) -> Vec<Vec2> {
    input
        .lines()
        .map(|l| l.split(",").map(|v| v.parse::<isize>().unwrap()).collect_tuple::<(isize, isize)>().unwrap().into())
        .collect()
}

fn successors(state: &Vec2, map: &SparseMap<char>, bounds: &Rect<isize>) -> Vec<(Vec2, usize)> {

    Direction::DIRECTION_4.into_iter().filter_map(|d| {
        let next = state.advance(&d);
        match map.get(&next) {
            None if bounds.is_inside(next) => Some((next, 1)),
            _ => None
        }
    }).collect()
}

fn part1(input: &str, size: (isize, isize), n: usize) -> usize {
    let bytes = parse(input);
    let bounds = Rect::new((0, 0), size);
    let target = Vec2::from(size);
    let mut map = SparseMap::new();
    bytes.into_iter().take(n).for_each(|v| {map.insert(v, '#');});

    let (_, cost) = astar(
        &(0,0).into(),
        |s| successors(s, &map, &bounds),
        |p| (target - *p).manhatan() as usize,
        |p| *p == target).unwrap();
    cost
}

fn part2(input: &str, size: (isize, isize)) -> Option<Vec2> {
    let bytes = parse(input);
    let bounds = Rect::new((0, 0), size);
    let target = Vec2::from(size);

    let mut l = 0;
    let mut r = bytes.len() - 1;
    while l < r {
        let t =  (l + r) / 2;
        let mut map = SparseMap::new();

        bytes.iter().take(t).for_each(|v| {map.insert(*v, '#');});

        if astar(
            &(0,0).into(),
            |s| successors(s, &map, &bounds),
            |p| (target - *p).manhatan() as usize,
            |p| *p == target).is_some() {
            l = t + 1;
        } else {
            r = t - 1;
        }

        if l == r && r != bytes.len() {
            return Some(bytes[t + 1]);
        }
    };
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day18_part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input, (6,6), 12), 22);
    }

    #[test]
    fn day18_part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input, (70, 70), 1024), 292);
    }

    #[test]
    fn day18_part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input, (6, 6)), Some((6, 1).into()));
    }

    #[test]
    fn day18_part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input, (70, 70)), Some((58, 44).into()));
    }
}
