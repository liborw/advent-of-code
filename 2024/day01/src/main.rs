use common::{aoc_task, took};
use itertools::Itertools;

fn main() {
    let input = include_str!("../input.txt");
    aoc_task!(|| part1(input));
    aoc_task!(|| part2(input));
}

fn parse(input: &str) -> (Vec<u32>, Vec<u32>) {
    input.lines()
         .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .collect_tuple()
                .unwrap()
        }).unzip()
}

fn part1(input: &str) -> u32 {
    let (mut a, mut b) = parse(input);

    a.sort_unstable();
    b.sort_unstable();

    a.into_iter()
     .zip(b)
     .map(|(i, j)| i.abs_diff(j))
     .sum()
}

fn part2(input: &str) -> u32 {
    let (a, b) = parse(input);
    let m = b.into_iter().counts();
    a.into_iter().map(|v| *m.get(&v).unwrap_or(&0) as u32 * v).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 11);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 2000468);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 31);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 18567089);
    }
}
