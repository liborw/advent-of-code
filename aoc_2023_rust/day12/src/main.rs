use std::{str::FromStr, fmt::Display, collections::HashMap};

use rayon::prelude::*;
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


fn parse(input: &str) -> Vec<(&str, Vec<usize>)> {
    input.lines()
         .map(|l| l.split_once(" ").unwrap())
         .map(|(s, c)| {
             (
                 s,
                 c.split(',').map(|s| s.parse().unwrap()).collect()
             )
         }).collect()
}

type Cache = HashMap<(usize, usize, usize), usize>;


fn ways(s: &[u8], check: &[usize], cur: Option<usize) -> usize {

    match s[0] {
        b'.' => ways(&s[1..], check, 0),
        b'#' => ways(&s[1..], check, cur + 1),
        b'?' => {
            let a = [b'#'].iter().chain(s[1..].iter()).collect();
            let b = [b'.'].iter().chain(s[1..].iter()).collect();
            ways(&a, check, cur) + ways(&b, check, cur)
        }
    }


}


fn part1(input: &str) -> usize {
    parse(input).into_par_iter().map(|(s, check)| {
        let mut cache = Cache::new();
        ways(s.as_bytes(), &check, 0)
    }).sum()
}

fn part2(input: &str) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_options() {

    }

    #[test]
    fn part1_test1() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 21);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 7118);
    }

    //#[test]
    //fn part2_test() {
    //    let input = include_str!("../input_test.txt");
    //    assert_eq!(part2(input), 525152);
    //}

    //#[test]
    //fn part2_final_test() {
    //    let input = include_str!("../input.txt");
    //    assert_eq!(part2(input), 1);
    //}
}
