use std::{collections::{HashMap, HashSet}, hash::Hash};

use itertools::Itertools;
use utils::{graph::maximal_cliques, run_task, took};

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input));
    run_task!(|| part2(input));
}

type Connections<'a> = HashMap<&'a str, HashSet<&'a str>>;

fn parse(input: &str) -> Connections {
    let mut cons: HashMap<&str, HashSet<&str>> = HashMap::new();
    input
        .lines()
        .for_each(|l| {
            let (a, b) = l.split_once("-").unwrap();
            cons.entry(a).or_default().insert(b);
            cons.entry(b).or_default().insert(a);
        });
    cons
}

fn intersect<T>(va: &HashSet<T>, vb: &HashSet<T>) -> Vec<T>
where T: Eq + Clone + Hash
{
    let mut out = Vec::new();
    for a in va.iter() {
        if vb.contains(a) {
            out.push(a.clone());
        }
    }
    out
}

fn triples<'a>(cons: &'a Connections) -> Vec<[&'a str;3]> {
    let mut triples = HashSet::new();

    cons.keys()
        .sorted()
        .tuple_combinations()
        .for_each(|(&a, &b)| {
            let a_cons = cons.get(&a).unwrap();

            if a_cons.contains(&b) {
                let b_cons = cons.get(&b).unwrap();

                for v in intersect(a_cons, b_cons) {
                    let mut triple = [a, b, v];
                    triple.sort();
                    triples.insert(triple);
                }
            }
        });
    triples.into_iter().collect()
}

fn part1(input: &str) -> usize {
    let cons = parse(input);
    let triples = triples(&cons);

    triples
        .into_iter()
        .filter(|v| v.iter().any(|s| s.starts_with('t')))
        .count()
}

fn part2(input: &str) -> String {
    let cons = parse(input);

    let cliques = maximal_cliques(&cons);
    let mut max_clique = cliques
        .into_iter()
        .max_by_key(|v| v.len())
        .unwrap();

    max_clique.sort();
    max_clique.into_iter().join(",")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day23_part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 7);
    }

    #[test]
    fn day23_part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 1400);
    }

    #[test]
    fn day23_part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), "co,de,ka,ta".to_string());
    }

    #[test]
    fn day23_part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), "am,bc,cz,dc,gy,hk,li,qf,th,tj,wf,xk,xo");
    }
}
