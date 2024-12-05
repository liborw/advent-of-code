use std::cmp::Ordering;

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
        .map(|l| l.split("|").map(|v| v.parse().unwrap()).collect_tuple::<(i32, i32)>().unwrap())
        .collect();

    (rules, updates)
}

fn check(update: &Update, rules: &Rules) -> bool {
    rules.iter().all(|(ia, ib)| {
        update.iter().position(|a| a == ia).unwrap_or(0) <= update.iter().position(|b| b == ib).unwrap_or(rules.len())
    })
}

fn part1(input: &str) -> i32 {
    let (rules, updates) = parse(input);
    updates
        .into_iter()
        .filter_map(|u| {
            check(&u, &rules).then_some(u[u.len()/2])
        }).sum()
}

fn sort(update: &mut Update, rules: &Rules) {
    update.sort_by(|a, b| {
        if let Some((c, _)) = rules.iter().find(|&&r| r == (*a, *b) || r == (*b, *a)) {
            if c == a {
                Ordering::Less
            } else {
                Ordering::Greater
            }
        } else {
            Ordering::Equal
        }
    });
}

fn part2(input: &str) -> i32 {
    let (rules, updates) = parse(input);
    updates
        .into_iter()
        .filter_map(|mut u| {
            if check(&u, &rules) {
                None
            } else {
                sort(&mut u, &rules);
                Some(u[u.len()/2])
            }
        }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sort_update_test() {
        let input = include_str!("../input_test.txt");
        let (rules, updates) = parse(input);
        let mut update = updates[3].clone();
        sort(&mut update, &rules);
        assert_eq!(update, vec![97,75,47,61,53]);
    }

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 143);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 5129);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 123);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 4077);
    }
}
