use std::collections::HashSet;

use common::{map::{SparseMap, Map}, pos::Pos};
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


fn parse(input: &str) -> SparseMap<char> {
    SparseMap::from_str(input, &|c| Some(c))
}

fn expand(map: &SparseMap<char>, pos: &Pos) -> Vec<Pos> {

    match map.get(pos).unwrap_or(&'#') {
            '<' => vec![*pos + (0, -1).into()],
            '>' => vec![*pos + (0,  1).into()],
            '^' => vec![*pos + (-1, 0).into()],
            'v' => vec![*pos + ( 1, 0).into()],
            '#' => vec![],
            '.' => pos.neighbors4(),
             _  => unreachable!()
    }
}

fn longest_path( map: &SparseMap<char>, start: &Pos, goal: &Pos) -> usize {
    let mut paths: Vec<(HashSet<Pos>, Pos)> = vec![(HashSet::from([*start]), *start)];

    while !paths.iter().all(|(_, p)| p == goal) {
        let mut new_paths: Vec<(HashSet<Pos>, Pos)> = Vec::new();
        for (visited, p) in paths.iter() {

            let neighbors: Vec<Pos> = expand(map, &p).into_iter().filter(|p| {
                map.get(p).unwrap_or(&'#') != &'#' && !visited.contains(p)
            }).collect();

            for p in neighbors.into_iter() {

                let mut new_visited = visited.clone();
                new_visited.insert(p);

                new_paths.push((new_visited, p))
            }
        }
        paths = new_paths;
    }

    paths.into_iter().map(|(hs, _)| hs.len()).max().unwrap() - 1
}


fn part1(input: &str) -> usize {
    let map = parse(input);
    let bb = map.bounding_box();
    let start = (0,1).into();
    let goal = (bb.x_max,bb.y_max - 1).into();
    longest_path(&map, &start, &goal)
}


fn expand2(map: &SparseMap<char>, pos: &Pos) -> Vec<Pos> {

    match map.get(pos).unwrap_or(&'#') {
            '#' => vec![],
             _  => pos.neighbors4(),
    }
}

fn longest_path2( map: &SparseMap<char>, start: &Pos, goal: &Pos) -> usize {
    let mut paths: Vec<(HashSet<Pos>, Pos)> = vec![(HashSet::from([*start]), *start)];
    let mut max_len = 0;

    while !paths.is_empty() {
        let mut new_paths: Vec<(HashSet<Pos>, Pos)> = Vec::new();
        for (visited, p) in paths.into_iter() {

            if p == *goal {
                if visited.len() - 1 >  max_len {
                    max_len = visited.len() - 1;
                }
                continue;
            }

            let neighbors: Vec<Pos> = expand2(map, &p).into_iter().filter(|p| {
                map.get(p).unwrap_or(&'#') != &'#' && !visited.contains(p)
            }).collect();

            // if neighbors.len() == 0 {
            //     let mut map = map.clone();
            //     visited.iter().for_each(|&v| {
            //         map.insert(v, '0');
            //     });
            //     map.dump('.');
            //     println!("");
            // }

            for p in neighbors.into_iter() {
                let mut new_visited = visited.clone();
                new_visited.insert(p);
                new_paths.push((new_visited, p))
            }
        }
        paths = new_paths;
    }

    max_len
}

fn part2(input: &str) -> usize {
    let map = parse(input);
    let bb = map.bounding_box();
    let start = (0,1).into();
    let goal = (bb.x_max,bb.y_max - 1).into();
    longest_path2(&map, &start, &goal)
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 94);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 2182);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 154);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1);
    }
}
