use std::collections::HashMap;

use itertools::Itertools;
use utils::{took, run_task};

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input));
    run_task!(|| part2(input));
}

type Graph<'a> = HashMap<&'a str, Wire<'a>>;
type Labels<'a> = HashMap<&'a str, (String, usize)>;


#[derive(Debug)]
enum Wire<'a> {
    Value(bool),
    And(&'a str, &'a str),
    Or(&'a str, &'a str),
    Xor(&'a str, &'a str),
}

impl Wire<'_> {

    fn inputs(&self) -> Option<(&str, &str)> {
        use Wire::*;
        match &self {
            Value(_) => None,
            And(a, b) => Some((*a, *b)),
            Or(a, b) => Some((*a, *b)),
            Xor(a, b) => Some((*a, *b)),
        }
    }

    fn has_input(&self, lbl: &str) -> bool {
        self.inputs().is_some_and(|(a, b)| a == lbl || b == lbl)
    }

    fn is_and(&self) -> bool {
        matches!(self, Wire::And(_, _))
    }

    fn is_or(&self) -> bool {
        matches!(self, Wire::Or(_, _))
    }

    fn is_xor(&self) -> bool {
        matches!(self, Wire::Xor(_, _))
    }

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
    let wire = graph.get(node).unwrap();
    match wire {
        Wire::Value(v) => *v,
        Wire::And(a, b) => eval(a, graph) && eval(b, graph),
        Wire::Or(a, b) => eval(a, graph) || eval(b, graph),
        Wire::Xor(a, b) => eval(a, graph) ^ eval(b, graph),
    }
}


fn get_value(label: &str, graph: &Graph) -> usize {
    graph
        .keys()
        .filter(|s|s.starts_with(label))
        .sorted()
        .map(|n| eval(n, graph))
        .enumerate()
        .filter_map(|(i, v)| {
            v.then_some(2usize.pow(i as u32))
        })
        .sum()
}

fn part1(input: &str) -> usize {
    let graph = parse(input);
    get_value("z", &graph)
}

fn try_split_label(label: &str) -> Option<(&str, usize)> {
    if label.starts_with("x") || label.starts_with("x") || label.starts_with("x") {
        Some((&label[0..1], label[1..].parse().unwrap()))
    } else {
        None
    }
}

fn find_by_input<'a>(lbl: &str, graph: &'a Graph) -> Vec<&'a str> {
    graph
        .iter()
        .filter(|(_, v)| v.has_input(lbl))
        .map(|(s, _)| *s)
        .collect()
}


fn part2(input: &str) -> String {
    let graph = parse(input);
    let mut wires: Vec<&str> = graph.keys().cloned().collect();
    let n_out = graph
        .keys()
        .filter(|z| z.starts_with("z"))
        .count();
    println!("n_bits_out: {n_out}");

    // remove all inputs
    wires.retain(|w| !(w.starts_with("x") || w.starts_with("y")));
    println!("N Wires to test: {}", wires.len());

    let mut errors: Vec<&str> = vec![];

    // get all Xor with directi inputs label theyr output as SAn (sum A)
    println!("Testing all input XOR:");
    wires.retain(|w| {
        let mut ret = true;
        let wire = graph.get(w).unwrap();
        if let Wire::Xor(a, b) = wire {
            if (a.starts_with("x") || a.starts_with("y")) && (b.starts_with("x") || b.starts_with("y")) {
                ret = false;

                let keys = find_by_input(w, &graph);
                if keys.len() != 2 {
                    if *w != "z00" {
                        println!("Wrong number of consumers: {wire:?} -> {w} -> {keys:?}");
                        errors.push(w);
                    }
                } else {

                    let out_a = graph.get(keys[0]).unwrap();
                    let out_b = graph.get(keys[1]).unwrap();

                    if !((out_a.is_and() && out_b.is_xor()) || (out_a.is_xor() && out_b.is_and())) {
                        println!("Wrong consumer types: {wire:?} -> {w} -> {out_a:?} {out_b:?}");
                        errors.push(w);
                    }
                };
            }
        }
        ret
    });

    // get all AND with direct inputs label their output as CAn (carry A)
    wires.retain(|w| {
        let mut ret = true;
        let wire = graph.get(w).unwrap();
        if let Wire::And(a, b) = wire {
            if (a.starts_with("x") || a.starts_with("y")) && (b.starts_with("x") || b.starts_with("y")) {
                ret = false;
                let keys = find_by_input(w, &graph);
                if keys.len() != 1 {
                    if *a != "x00" {
                        println!("Wrong number of consumers: {wire:?} -> {w} -> {keys:?}");
                        errors.push(w);
                    }
                } else {
                    let wtest = graph.get(keys[0]).unwrap();
                    if !wtest.is_or() {
                        println!("Wrong consumer: {wire:?} -> {w} -> {wtest:?}");
                        errors.push(w);
                    }
                };
            }
        }
        ret
    });

    // get all OR and check whether one of inputs already has label
    wires.retain(|w| {
        let mut ret = true;
        let wire = graph.get(w).unwrap();
        if let Wire::Or(_, _) = wire {
            ret = false;
            let keys = find_by_input(w, &graph);
            if keys.len() != 2 {
                if *w != "z45" {
                    println!("Wrong number of consumers: {wire:?} -> {w} -> {keys:?}");
                    errors.push(w);
                }
            } else {
                let out_a = graph.get(keys[0]).unwrap();
                let out_b = graph.get(keys[1]).unwrap();

                if !((out_a.is_and() && out_b.is_xor()) || (out_a.is_xor() && out_b.is_and())) {
                    println!("Wrong consumer types: {wire:?} -> {w} -> {out_a:?} {out_b:?}");
                    errors.push(w);
                }
            };
        }
        ret
    });


    // get rest of the XOR and check their output correct
    wires.retain(|w| {
        let mut ret = true;
        let wire = graph.get(w).unwrap();
        if let Wire::Xor(_, _) = wire {
            ret = false;
            let keys = find_by_input(w, &graph);
            if !keys.is_empty() {
                println!("Wrong number of consumers: {wire:?} -> {w} -> {keys:?}");
                errors.push(w);
            };

        }
        ret
    });

    wires.retain(|w| {
        let mut ret = true;
        let wire = graph.get(w).unwrap();
        if let Wire::And(_, _) = graph.get(w).unwrap() {
            ret = false;
            let keys = find_by_input(w, &graph);
            if keys.len() != 1 {
                println!("Wrong number of consumers: {wire:?} -> {w} -> {keys:?}");
                errors.push(w);
            } else {
                let wtest = graph.get(keys[0]).unwrap();
                if !wtest.is_or() {
                    println!("Wrong consumer: {wire:?} -> {w} -> {wtest:?}");
                    errors.push(w);
                }
            };
        }
        ret
    });

    errors.into_iter().sorted().join(",")
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
    fn day24_part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), "cgr,hpc,hwk,qmd,tnt,z06,z31,z37");
    }
}
