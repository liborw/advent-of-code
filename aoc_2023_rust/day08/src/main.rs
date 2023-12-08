use std::collections::HashMap;

use rayon::iter::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
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

type Graph = HashMap<String, (String, String)>;

fn parse(input: &str) -> (Vec<char>, Graph) {
    let mut lines = input.lines();
    let moves = lines.next().unwrap().chars().collect();
    let graph = lines.skip(1).map(|l| {
        (l[0..3].to_string(), (l[7..10].to_string(), l[12..15].to_string()))
    }).collect();
    (moves, graph)
}

fn part1(input: &str) -> usize {
    let (moves, graph) = parse(input);
    let mut node = "AAA";
    let mut n = 0;

    for m in moves.iter().cycle() {
        let (l, r) = graph.get(node).unwrap();

        match m {
            'R' => node = r,
            'L' => node = l,
             _  => panic!()
        }
        n += 1;

        if node == "ZZZ" {
            break
        }
    }
    n
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}


fn part2(input: &str) -> usize {
    let (moves, graph) = parse(input);
    let nodes: Vec<String> = graph.keys().cloned().filter(|k| k.ends_with('A')).collect();

    // (start, length)
    let repeats: Vec<(usize, usize, Vec<usize>)> = nodes.par_iter().map(|n| {
        let mut states: HashMap<(String, usize), usize> = HashMap::new();
        let mut node = n.clone();
        let mut goal: Vec<usize> = Vec::new();

        // i - overall step
        // j - step in the moves array
        moves.iter().enumerate().cycle().enumerate().find_map(|(i, (j, m))| {
            if &node[2..] == "Z" {
                goal.push(i);
            }
            //println!("{i:?}: {node:?} {j:?} {states:?}");
            if let Some(v) = states.get(&(node.clone(), j)) {
                Some((*v, i - v, goal.clone()))
            } else {
                states.insert((node.to_string(), j), i);

                // make the step
                let (l, r) = graph.get(&node).unwrap();
                node = match m {
                    'R' => r.clone(),
                    'L' => l.clone(),
                     v  => panic!("{}", v)
                };
                None
            }
        }).unwrap()
    }).collect();

    repeats.iter().map(|(_, _, v)| *v.first().unwrap()).reduce(|acc, v| lcm(acc, v)).unwrap()
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test01() {
        let input = include_str!("../input_test01.txt");
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn part1_test02() {
        let input = include_str!("../input_test02.txt");
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test03.txt");
        assert_eq!(part2(input), 6);
    }
}
