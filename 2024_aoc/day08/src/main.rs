
use std::collections::{hash_map::Entry, HashMap};

use itertools::Itertools;
use utils::{aoc_task, map::{Map, SparseMap}, took, vector::Vec2};



fn main() {
    let input = include_str!("../input.txt");
    aoc_task!(|| part1(input));
    aoc_task!(|| part2(input));
}

fn parse(input: &str) -> SparseMap<char> {
    SparseMap::from_str(input, &|c| Some(c))
}

fn part1(input: &str) -> usize {
    let map = parse(input);
    let mut antinodes = map.copy_map(|_| Some('.'));

    map.print('x');

    // get all antenas
    let antenas: HashMap<char, Vec<Vec2<isize>>> = map.into_iter().sorted_by_key(|(p, _)| *p)
        .filter(|(_, v)| v != &'.')
        .fold(HashMap::new(), |mut m, (p, v)| {
            m.entry(v).or_default().push(p);
            m
        });
    println!("{:?}", antenas);

    antenas.values().for_each(|antenas| {
        antenas.iter().combinations(2).for_each(|pair|{
            let a = *pair[0];
            let b = *pair[1];


            let v = if a > b {
                a - b
            } else {
                b - a
            };

            let na = a - v;
            antinodes.entry(na).and_modify(|v| *v = '#');


            let nb = b + v;
            antinodes.entry(nb).and_modify(|v| *v = '#');
        });
    });


    println!();
    antinodes.print('x');

    antinodes.into_values().filter(|a| a == &'#' ).count()
}

fn part2(input: &str) -> usize {
    let map = parse(input);
    let mut antinodes = map.copy_map(|_| Some('.'));

    map.print('x');

    // get all antenas
    let antenas: HashMap<char, Vec<Vec2<isize>>> = map.into_iter().sorted_by_key(|(p, _)| *p)
        .filter(|(_, v)| v != &'.')
        .fold(HashMap::new(), |mut m, (p, v)| {
            m.entry(v).or_default().push(p);
            m
        });
    println!("{:?}", antenas);

    antenas.values().for_each(|antenas| {
        antenas.iter().combinations(2).for_each(|pair|{
            let a = *pair[0];
            let b = *pair[1];


            let v = if a > b {
                a - b
            } else {
                b - a
            };

            for i in 0.. {
                let n = a - v * i;
                if let Entry::Occupied(mut e) = antinodes.entry(n) {
                    e.insert('#');
                } else {
                    break
                }
            };

            for i in  0.. {
                let n = b + v * i;
                if let Entry::Occupied(mut e) = antinodes.entry(n) {
                    e.insert('#');
                } else {
                    break
                }
            };
        });
    });


    println!();
    antinodes.print('x');

    antinodes.into_values().filter(|a| a == &'#' ).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 14);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 381);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 34);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1184);
    }
}
