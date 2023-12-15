use std::collections::HashMap;
use std::fmt::Debug;
use std::hash::Hash;

use took::took;
use common::map::*;
use common::pos::*;
use itertools::Itertools;

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


pub fn repeat<T: Debug, K: Eq + Hash>(
    mut state: T,
    mut next: impl FnMut(T) -> T,
    mut key_fn: impl FnMut(&T) -> K,
    n: usize
) -> T {
    let mut hist: HashMap<K, usize> = HashMap::new();

    for i in 0..n {
        let key = key_fn(&state);
        if let Some(offset) = hist.get(&key) {
            let period = i - offset;
            let n_left = (n - i) % period;
            for _ in 0..n_left {
                state = next(state);
            }
            return state;
        } else {
            hist.insert(key, i);
            state = next(state);
        }
    }
    state
}


fn parse(input: &str) -> SparseMap<char>  {
    input.lines().enumerate().map(|(x, l)| {
        l.chars().enumerate().filter_map(move |(y, c)| {
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
                for y in (0..=bb.y_max).rev() {
                    let key = (x, y).into();
                    match (map.get(&key), first_empty) {
                        (Some('O'), None) => (),
                        (Some('O'), Some(p)) => {
                            map.remove(&key);
                            map.insert(p, 'O');
                            first_empty = first_empty.map(|p| p + (0, -1).into());
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

fn map_state_key(map: &SparseMap<char>) -> String {
    let mut hash = String::new();
    map.iter()
       .filter_map(|(p, &c)| {
           if c == 'O' {
               Some(p)
           } else {
               None
           }
       }).sorted().for_each(|p| {
           hash.push_str(format!("{}-{},", p.x, p.y).as_str());
       });
    hash
}

fn part2(input: &str) -> isize {
    let mut map = parse(input);
    println!("Start:");
    map.dump('.');
    map = repeat(map,
                 |mut map| {
                    roll(&mut map, &Direction::North);
                    roll(&mut map, &Direction::West);
                    roll(&mut map, &Direction::South);
                    roll(&mut map, &Direction::East);
                    println!("\nEast *:");
                    map.dump('.');
                    map
                 },
                 map_state_key,
                 1000000000
                 );
    load(&map)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn repeat_test() {
        let state = "abcabcabc".as_bytes();
        let a = repeat(state, |s| &s[1..], |s| s[0], 7);
        assert_eq!(a[0], state[7]);
    }

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
        assert_eq!(part2(input), 93742);
    }
}
