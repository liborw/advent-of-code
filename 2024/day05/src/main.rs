use std::{collections::HashMap, ops::Index};

use itertools::Itertools;
use utils::{took, aoc_task};



fn main() {
    let input = include_str!("../input.txt");
    aoc_task!(|| part1(input));
    aoc_task!(|| part2(input));
}

type Rules = Vec<(i32, i32)>;
type Update = Vec<i32>;
type Updates = Vec<Update>;

fn parse(input: &str) -> (Rules, Updates) {
    let (rules_str, updates_str) = input.split_once("\n\n").unwrap();

    let updates = updates_str
        .lines()
        .map(|line| {
            line.split(",")
                .map(|v| v.parse().unwrap()).collect()
        })
        .collect();

    let rules = rules_str
        .lines()
        .map(|l| l.split("|").map(|v| v.parse().unwrap()).collect_tuple().unwrap())
        .collect();

    (rules, updates)
}

fn check(update: &Update, rules: &Rules) -> bool {
    rules.iter().all(|(ia, ib)| {
        update.iter().position(|a| a == ia).unwrap_or(0) <= update.iter().position(|b| b == ib).unwrap_or(rules.len())
    })
}

fn mid(update: Update) -> i32 {
    update[update.len()/2]
}


fn part1(input: &str) -> i32 {
    let (rules, updates) = parse(input);

    updates
        .into_iter()
        .filter_map(|u| {
            check(&u, &rules).then_some(mid(u))
        }).sum()
}

fn part2(input: &str) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 143);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1);
    }
}
