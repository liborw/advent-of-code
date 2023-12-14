use std::{str::FromStr, fmt::Display, collections::HashMap, iter::once};

use itertools::Itertools;
use rayon::prelude::*;
use took::took;
use memoize::memoize;

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


#[memoize]
fn ways(s: &[u8], check: &[usize], cur: Option<usize>) -> usize {

    if s.is_empty() {
        return match (cur, check.len()) {
            (None, 0) => 1,
            (Some(x), 1) if x == check[0] => 1,
            _ => 0
        }
    }

    if cur.is_some() && check.is_empty() {
        return 0;
    }

    match (s[0], cur) {
        (b'.', Some(x)) if x != check[0] => 0,
        (b'.', Some(_)) => ways(&s[1..], &check[1..], None),
        (b'.', None)    => ways(&s[1..], check, None),
        (b'#', Some(x)) => ways(&s[1..], check, Some(x+1)),
        (b'#', None)    => ways(&s[1..], check, Some(1)),
        (b'?', Some(x)) => {
            let ans = ways(&s[1..], check, Some(x+1));
            if x == check[0] {
                ans + ways(&s[1..], &check[1..], None) // has .
            } else {
                ans
            }
        },
        (b'?', None) => ways(&s[1..], check, Some(1)) + ways(&s[1..], check, None),
        _ => unreachable!()
    }
}


fn part1(input: &str) -> usize {
    parse(input).into_par_iter().map(|(s, check)| {
        ways(s.as_bytes(), &check, None)
    }).sum()
}

fn part2(input: &str) -> usize {
    parse(input).into_par_iter().map(|(s, check)| {
        let s = once(s).cycle().take(5).join("?");
        let check = once(check).cycle().take(5).flatten().collect::<Vec<usize>>();
        let n = ways(s.as_bytes(), &check, None);
        println!("{s}, {check:?} => {n}");
        n
    }).sum()
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

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 525152);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1);
    }
}
