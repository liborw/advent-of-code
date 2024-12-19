use std::collections::HashMap;

use utils::{took, run_task};

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input));
    run_task!(|| part2(input));
}

struct Puzzle<'a> {
    towels: Vec<&'a str>,
    patterns: Vec<&'a str>
}

fn parse(input: &str) -> Puzzle {
    let (tstr, pstr) = input.split_once("\n\n").unwrap();

    let towels = tstr.split(", ").collect();
    let patterns = pstr.lines().collect();

    Puzzle{towels, patterns}
}


fn is_possible<'a>(pattern: &'a str, towels: &[&str], cache: &mut HashMap<&'a str, usize>) -> usize {
    if pattern.is_empty() {
        return 1;
    }

    if let Some(v) = cache.get(pattern) {
        return *v;
    }

    let possible = towels.iter().map(|t| {
        if let Some(newpat) = pattern.strip_prefix(t) {
            is_possible(newpat, towels, cache)
        } else {
            0
        }
    }).sum();

    cache.insert(pattern, possible);

    possible
}

// 30 min
fn part1(input: &str) -> usize {
    let puzzle = parse(input);
    let mut cache = HashMap::new();

    puzzle.patterns.iter().filter(|p| is_possible(p, &puzzle.towels, &mut cache) > 0).count()
}

// 5 min
fn part2(input: &str) -> usize {
    let puzzle = parse(input);
    let mut cache = HashMap::new();

    puzzle.patterns.iter().map(|p| is_possible(p, &puzzle.towels, &mut cache)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day19_part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn day19_part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 363);
    }

    #[test]
    fn day19_part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 16);
    }

    #[test]
    fn day19_part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 642535800868438);
    }
}
