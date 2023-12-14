use std::collections::HashMap;

use took::took;
use common::map::*;
use common::pos::*;

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

fn parse(input: &str) -> SparseMap<char>  {
    input.lines().enumerate().map(|(x,l)| {
        l.chars().enumerate().filter_map(move |(y,c)| {
            if c != '.' {
                Some(((x,y).into(), c))
            } else {
                None
            }
        })
    }).flatten().collect()
}

#[derive(Debug)]
enum Direction {
    North,
    West,
    South,
    East,
}

fn roll(map: &mut SparseMap<char>, dir: &Direction) -> () {
    let bb  = map.bounding_box();
    let mut first_empty: Option<Pos>;


    match dir {
        Direction::North => {
            for y in 0..bb.y_max + 1 {
                first_empty = None;
                for x in 0..bb.x_max + 1 {
                    let key = (x, y).into();
                    match (map.get(&key), first_empty) {
                        (Some('O'), None) => (),
                        (Some('O'), Some(p)) => {
                            map.remove(&key);
                            map.insert(p, 'O');
                            first_empty = first_empty.map(|p| p + (1, 0).into());
                        },
                        (None, None) => first_empty = Some(key),
                        (None, _) => (),
                        (Some('#'), _) => first_empty = None,
                        s => unreachable!("Unreachable state: {s:?}")
                    }
                }
            }
        },

        Direction::South => {
            for y in 0..=bb.y_max {
                first_empty = None;
                for x in (0..=bb.x_max).rev() {
                    let key = (x, y).into();
                    match (map.get(&key), first_empty) {
                        (Some('O'), None) => (),
                        (Some('O'), Some(p)) => {
                            map.remove(&key);
                            map.insert(p, 'O');
                            first_empty = first_empty.map(|p| p + (-1, 0).into());
                        },
                        (None, None) => first_empty = Some(key),
                        (None, _) => (),
                        (Some('#'), _) => first_empty = None,
                        s => unreachable!("Unreachable state: {s:?}")
                    }
                }
            }
        },

        Direction::East => {
            for x in 0..=bb.x_max {
                first_empty = None;
                for y in 0..=bb.y_max {
                    let key = (x, y).into();
                    match (map.get(&key), first_empty) {
                        (Some('O'), None) => (),
                        (Some('O'), Some(p)) => {
                            map.remove(&key);
                            map.insert(p, 'O');
                            first_empty = first_empty.map(|p| p + (0, 1).into());
                        },
                        (None, None) => first_empty = Some(key),
                        (None, _) => (),
                        (Some('#'), _) => first_empty = None,
                        s => unreachable!("Unreachable state: {s:?}")
                    }
                }
            }
        },

        Direction::West => {
            for x in 0..=bb.x_max {
                first_empty = None;
                for y in (0..=bb.y_max).rev() {
                    let key = (x, y).into();
                    match (map.get(&key), first_empty) {
                        (Some('O'), None) => (),
                        (Some('O'), Some(p)) => {
                            map.remove(&key);
                            map.insert(p, 'O');
                            first_empty = first_empty.map(|p| p + (0, 1).into());
                        },
                        (None, None) => first_empty = Some(key),
                        (None, _) => (),
                        (Some('#'), _) => first_empty = None,
                        s => unreachable!("Unreachable state: {s:?}")
                    }
                }
            }
        },
    }
}

fn load(map: &SparseMap<char>) -> isize {
    let bb  = map.bounding_box();
    map.iter().filter_map(|(p, &c)| {
        if c == 'O' {
            Some(bb.x_max - p.x + 1)
        } else {
            None
        }
    }).sum()
}

fn part1(input: &str) -> isize {
    let mut map = parse(input);
    roll(&mut map, &Direction::North);
    load(&map)
}

fn hash(map: &SparseMap<char>) -> String {

    map.iter().filter_map(|k, c| {
        if
    }).sorted


}

fn part2(input: &str) -> isize {
    let hist: HashMap<String, usize> = HashMap::new();
    let mut map = parse(input);
    [Direction::North, Direction::West, Direction::South, Direction::East].iter()
        .cycle().take(1000000000*4).enumerate().find(|(j, dir)| {
            roll(&mut map, dir);
            let key = hash(&map);
            if let Some(i) = hist.get(&key) {
                Some(i)
            } else {
                hist.insert(key, j)
            }

    });

    load(&map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 136);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 105003);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 64);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1);
    }
}
