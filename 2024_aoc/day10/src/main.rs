use pathfinding::{matrix::Matrix, prelude::bfs_reach};
use utils::{direction::{cardinal::Direction, AdvanceInDirection}, map::{Map, SparseMap}, run_task, took};

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input));
    run_task!(|| part2(input));
}

fn parse(input: &str) -> SparseMap<i8> {
    SparseMap::from_str(input, &|c| c.to_digit(10).map(|d| d as i8))
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    let starts = map.find_all(&|d| d == &0);

    starts.into_iter().map(|s| {
        bfs_reach(s, |p| {
            let cur = map.get(p).unwrap();

            Direction::ALL.into_iter().filter_map(|d| {
                let next = p.advance(&d);
                map.get(&next).is_some_and(|&v| v - 1 == *cur).then_some(next)
            }).collect::<Vec<_>>()
        }).filter(|v| map.get(v).is_some_and(|&v| v == 9)).count()
    }).sum()

}

fn part2(input: &str) -> usize {
    0
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
        assert_eq!(part2(input), 1);
    }
}
