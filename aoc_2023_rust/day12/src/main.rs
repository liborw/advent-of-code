use std::{str::FromStr, fmt::Display};

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

#[derive(Debug, PartialEq, PartialOrd, Ord, Eq, Clone, Copy)]
enum Spring{
    Ok,
    Damaged,
    MaybeOk,
    MaybeDamaged
}

impl Spring {
    fn is_damaged(&self) -> bool {
        match self {
            Spring::MaybeOk | Spring::Ok => false,
            Spring::MaybeDamaged | Spring::Damaged => true,
        }
    }

    fn char(&self) -> char {
        match self {
            Spring::Ok => 'O',
            Spring::Damaged => 'D',
            Spring::MaybeOk => 'o',
            Spring::MaybeDamaged => 'd',
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Springs(Vec<Spring>);

impl From<Vec<Spring>> for Springs {
    fn from(value: Vec<Spring>) -> Self {
        Springs(value)
    }
}

impl FromStr for Springs {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
         Ok(s.chars().map(|c| {
             match c {
                 '.' => Spring::Ok,
                 '#' => Spring::Damaged,
                 '?' => Spring::MaybeOk,
                  v  => unreachable!("Unexpected character: {v}"),
             }
         }).collect::<Vec<_>>().into())
    }

}

impl Iterator for Springs {
    type Item = Springs;

    fn next(&mut self) -> Option<Self::Item> {
        for i in 0..self.0.len() {
            match self.0[i] {
                Spring::MaybeOk => {
                    self.0[i] = Spring::MaybeDamaged;
                    return Some(self.clone())
                },
                Spring::MaybeDamaged => {
                    self.0[i] = Spring::MaybeOk;
                }
                _ => ()
            }
        }
        None
    }
}

impl Display for Springs {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0.iter().map(|s| s.char()).collect::<String>())
    }
}

impl Springs {
    fn damaged_blocks(&self) -> Vec<u32> {
        self.0.split(|s| !s.is_damaged()).map(|v| v.len() as u32).filter(|v| *v > 0).collect()
    }

    fn count_options(&self, blocks: &Vec<u32>) -> usize {
        let s = self.clone();
        let c = s.filter(|v| &v.damaged_blocks() == blocks).count();

        if &self.damaged_blocks() == blocks {
            c + 1
        } else {
            c
        }
    }
}

fn parse(input: &str) -> Vec<(Springs, Vec<u32>)> {
    input.lines()
         .map(|l| l.split_once(" ").unwrap())
         .map(|(s, c)| {
             (
                 s.parse().unwrap(),
                 c.split(',').map(|s| s.parse().unwrap()).collect()
             )
         }).collect()
}


fn part1(input: &str) -> usize {
    parse(input).into_par_iter().map(|(s, check)| {
        s.count_options(&check)
    }).sum()
}

fn part2(input: &str) -> usize {
    parse(input).into_par_iter().map(|(s, check)| {
        let old_s = s.clone();
        let c = s.filter(|v| v.damaged_blocks() == check).count();
        if old_s.damaged_blocks() == check {
            c + 1
        } else {
            c
        }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_test() {
        let mut springs = Springs::from_str("??").unwrap();
        assert_eq!(springs.next(), Some(vec![Spring::MaybeDamaged, Spring::MaybeOk].into()));
        assert_eq!(springs.next(), Some(vec![Spring::MaybeOk, Spring::MaybeDamaged].into()));
        assert_eq!(springs.next(), Some(vec![Spring::MaybeDamaged, Spring::MaybeDamaged].into()));
        assert_eq!(springs.next(), None);
    }

    #[test]
    fn damaged_blocks() {
        let springs = Springs::from_str("##.##...###").unwrap();
        assert_eq!(springs.damaged_blocks(), vec![2,2,3]);
    }

    #[test]
    fn count_options() {
        let springs = Springs::from_str("????.######..#####.").unwrap();
        let blocks = vec![1, 6, 5];
        assert_eq!(springs.count_options(&blocks), 4);
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
