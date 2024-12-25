use std::{collections::HashMap};

use itertools::Itertools;
use utils::{took, run_task};

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input));
    run_task!(|| part2(input));
}

type Graph<'a> = HashMap<&'a str, Wire<'a>>;

#[derive(Debug)]
enum Wire<'a> {
    Value(bool),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
}


fn parse(input: &str) -> Graph {
    let mut map = HashMap::new();
    let (inputs, gates) = input.split_once("\n\n").unwrap();

    for l in inputs.lines() {
        let (name, value) = l.split_once(": ").unwrap();
        map.insert(name, Wire::Value(value == "1"));
    }

    for g in gates.lines() {
        let parts: Vec<_> = g.split_whitespace().collect();

        let gate = match parts[1] {
            "XOR" => Wire::Xor(parts[0], parts[2]),
            "OR" => Wire::Or(parts[0], parts[2]),
            "AND" => Wire::And(parts[0], parts[2]),
            _ => unreachable!(),
        };

        map.insert(parts[4], gate);


    }
    map
}

fn eval(node: &str, graph: &Graph) -> bool {
    match graph.get(node).unwrap() {
        Wire::Value(v) => *v,
        Wire::And(a, b) => eval(a, graph) && eval(b, graph),
        Wire::Or(a, b) => eval(a, graph) || eval(b, graph),
        Wire::Xor(a, b) => eval(a, graph) ^ eval(b, graph),
    }
}


fn part1(input: &str) -> usize {
    let graph = parse(input);
    println!("{graph:?}");
    graph
        .keys()
        .filter(|s| s.starts_with("z"))
        .sorted()
        .map(|n| eval(n, &graph))
        .enumerate()
        .filter_map(|(i, v)| {
            v.then_some(2usize.pow(i as u32))
        })
        .sum()
}

fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day24_part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn day24_part1_test2() {
        let input = include_str!("../input_test2.txt");
        assert_eq!(part1(input), 2024);
    }

    #[test]
    fn day24_part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 60614602965288);
    }

    #[test]
    fn day24_part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn day24_part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1);
    }
}
