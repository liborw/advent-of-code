use std::{collections::{HashMap, HashSet}, hash::Hash};

use itertools::Itertools;
use utils::{took, run_task};

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

// algorithm BronKerbosch2(R, P, X) is
//    if P and X are both empty then
//        report R as a maximal clique
//    choose a pivot vertex u in P ⋃ X
//    for each vertex v in P \ N(u) do
//        BronKerbosch2(R ⋃ {v}, P ⋂ N(v), X ⋂ N(v))
//        P := P \ {v}
//        X := X ⋃ {v}

fn bron_kerbosch<T>(
    r: &HashSet<T>,
    p: &mut HashSet<T>,
    x: &mut HashSet<T>,
    g: &HashMap<T, HashSet<T>>,
    cliques: &mut Vec<Vec<T>>,)
where T: Clone + Eq + Hash + Ord
{
    if p.is_empty() && x.is_empty() {
        if r.len() > 2 {
            let mut clique: Vec<T> = r.iter().cloned().collect();
            clique.sort();
            cliques.push(clique);
        }
        return;
    }

    // Choose a pivot with the maximum degree in P ∪ X
    let pivot = p
        .union(x)
        .max_by_key(|v| g.get(*v).map_or(0, |neighbors| neighbors.len()))
        .cloned();

    if let Some(pivot_vertex) = pivot {
        let neighbors = g.get(&pivot_vertex).cloned().unwrap_or_default();
        let candidates: Vec<T> = p.difference(&neighbors).cloned().collect();

        for v in candidates {
            // New R is R ∪ {v}
            let mut new_r = r.clone();
            new_r.insert(v.clone());

            // New P is P ∩ N(v)
            let neighbors_v = g.get(&v).cloned().unwrap_or_default();
            let mut new_p = p.intersection(&neighbors_v).cloned().collect::<HashSet<T>>();

            // New X is X ∩ N(v)
            let mut new_x = x.intersection(&neighbors_v).cloned().collect::<HashSet<T>>();

            // Recursive call
            bron_kerbosch(&new_r, &mut new_p, &mut new_x, g, cliques);

            // Move v from P to X
            p.remove(&v);
            x.insert(v);
        }
    }
}

fn part2(input: &str) -> String {
    let cons = parse(input);

    // Initialize R, P, X
    let r: HashSet<_> = HashSet::new();
    let mut p: HashSet<_> = cons.keys().cloned().collect();
    let mut x: HashSet<_> = HashSet::new();

    // Collect cliques
    let mut cliques: Vec<Vec<_>> = Vec::new();
    bron_kerbosch(&r, &mut p, &mut x, &cons, &mut cliques);

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
