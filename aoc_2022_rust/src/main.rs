#![allow(dead_code)]

use std::{collections::HashMap, str::FromStr};
use itertools::Itertools;
use std::num::ParseIntError;
use took::took;
use regex::Regex;

// aoc_task macro {{{

macro_rules! aoc_task {
    ($f:ident) => {
        let (took, result) = took($f);
        println!("{} took: {} result: {}", stringify!($f), took, result);
    };
}
// }}}
// main {{{

fn main() {
    aoc_task!(day01a);
    aoc_task!(day01b);
    aoc_task!(day02a);
    aoc_task!(day02b);
    aoc_task!(day03a);
    aoc_task!(day03b);
    aoc_task!(day04a);
    aoc_task!(day04b);
    aoc_task!(day05a);
    aoc_task!(day05b);
    aoc_task!(day06a);
    aoc_task!(day06b);
    aoc_task!(day07a);
    aoc_task!(day07b);
}

// }}}
// day01a {{{

fn day01a() -> i32 {
    include_str!("../input/day01.txt")
        .split("\n\n")
        .map(|e| {
            e.lines()
             .map(|v| v.parse::<i32>().unwrap())
             .sum()
        }).max().unwrap()
}

// }}}
// day01b {{{

fn day01b() -> i32 {
     let mut elves: Vec<_> = include_str!("../input/day01.txt")
        .split("\n\n")
        .map(|e| {
            e.lines()
             .map(|v| v.parse::<i32>().unwrap())
             .sum()
        }).collect();

    elves.sort();
    elves.reverse();
    elves.iter().take(3).sum()
}

// }}}
// day02a {{{

// A X Rock (1)
// B Y Paper (2)
// C Z Sizzors (3)
//

fn day02a() -> i32 {
     include_str!("../input/day02.txt")
         .lines()
         .map(|l| {
             match l {
                 "A X" => 1 + 3,
                 "A Y" => 2 + 6,
                 "A Z" => 3 + 0,
                 "B X" => 1 + 0,
                 "B Y" => 2 + 3,
                 "B Z" => 3 + 6,
                 "C X" => 1 + 6,
                 "C Y" => 2 + 0,
                 "C Z" => 3 + 3,
                 _ => unreachable!()
             }
         }).sum()
}

// }}}
// day02b {{{

// A Rock (1)
// B Paper (2)
// C Sizzors (3)
// X lose (0)
// Y draw (3)
// Z win (6)

fn day02b() -> i32 {
     include_str!("../input/day02.txt")
         .lines()
         .map(|l| {
             match l {
                 "A X" => 3 + 0,
                 "A Y" => 1 + 3,
                 "A Z" => 2 + 6,
                 "B X" => 1 + 0,
                 "B Y" => 2 + 3,
                 "B Z" => 3 + 6,
                 "C X" => 2 + 0,
                 "C Y" => 3 + 3,
                 "C Z" => 1 + 6,
                 _ => unreachable!()
             }
         }).sum()
}

// }}}
// day03a {{{

fn first_common_char(a: &str, b: &str) -> Option<char> {
    a.chars().find(|&ch| b.contains(ch))
}


fn day03a() -> i32 {

    let map: HashMap<char, i32> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().zip(1..=52).collect();
    include_str!("../input/day03.txt")
        .lines()
        .map(|l| {
            let (a, b) = l.split_at(l.len()/2);
            let ch = first_common_char(a, b).unwrap();
            map.get(&ch).unwrap()
        }).sum()
}

// }}}
// day03b {{{

fn common_to_three(a: &str, b: &str, c: &str) -> Option<char> {
    a.chars().find(|&ch| b.contains(ch) & c.contains(ch))
}

fn day03b() -> i32 {

    let map: HashMap<char, i32> = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ".chars().zip(1..=52).collect();
    let lines: Vec<&str> = include_str!("../input/day03.txt") .lines().collect();

    lines[..].chunks(3).map(|chunk| {
        let ch = common_to_three(chunk[0], chunk[1], chunk[2]).unwrap();
        map.get(&ch).unwrap()
    }).sum()
}

// }}}
// day04a {{{

struct Range {
    from: i32,
    to: i32
}

impl FromStr for Range {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (from, to) = s.split_once('-').unwrap();
        let from = from.parse::<i32>()?;
        let to = to.parse::<i32>()?;

        Ok(Range {
            from,
            to
        })
    }
}

fn day04a() -> i32 {

    include_str!("../input/day04.txt")
        .lines()
        .map(|l| {
            let (r1_str, r2_str) = l.split_once(',').unwrap();
            let r1:Range = r1_str.parse().unwrap();
            let r2:Range = r2_str.parse().unwrap();
            i32::from((r1.from <= r2.from) & (r1.to >= r2.to) || (r2.from <= r1.from) & (r2.to >= r1.to))
        }).sum()
}

fn day04a_alt() -> i32 {

    include_str!("../input/day04.txt")
        .lines()
        .map(|l| {
            let v:Vec<i32> = l.split(&['-', ','][..])
                                .map(|v| v.parse().unwrap())
                                .collect();
            let (x1, y1, x2, y2) = (v[0], v[1], v[2], v[3]);
            i32::from(x1 <= x2 && y1 >= y2 || x2 <= x1 && y2 >= y1)
        }).sum()
}

// }}}
// day04b {{{


fn day04b() -> i32 {

    include_str!("../input/day04.txt")
        .lines()
        .map(|l| {
            let (r1_str, r2_str) = l.split_once(',').unwrap();
            let r1:Range = r1_str.parse().unwrap();
            let r2:Range = r2_str.parse().unwrap();

            i32::from(r1.from <= r2.to && r1.to >= r2.from)
        }).sum()

}

fn day04b_alt() -> i32 {

    include_str!("../input/day04.txt")
        .lines()
        .map(|l| {
            let v:Vec<i32> = l.split(&['-', ','][..])
                              .map(|v| v.parse().unwrap())
                              .collect();
            let (x1, y1, x2, y2) = (v[0], v[1], v[2], v[3]);
            i32::from(x1 <= y2 && y1 >= x2)
        }).sum()
}

// }}}
// day05a {{{

#[derive(Debug)]
struct WarehouseState {
    piles: Vec<Vec<char>>
}

impl WarehouseState {
    fn move_crate(&mut self, n:usize, from:usize, to: usize) {

        for _ in 0..n {
            let cr = self.piles[from-1].pop().unwrap();
            self.piles[to-1].push(cr);
        }
    }

    fn top_word(&self) -> String {
        let mut s = String::new();

        for pile in &self.piles {
            s.push(*pile.last().unwrap());
        }
        s
    }
}

impl FromStr for WarehouseState {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines()
                             .map(|l| l.chars().enumerate().skip(1).fold(Vec::new(), |mut acc, (i, ch)| {
                                 if (i - 1) % 4 ==  0 {
                                    acc.push(ch);
                                 }
                                 acc
                             } ))
                             .rev()
                             .collect();

        let mut piles: Vec<Vec<char>> = Vec::new();
        for _ in &lines[0] {
            piles.push(Vec::new());
        }

        for l in lines.iter().skip(1) {
            for (i, ch) in l.iter().enumerate() {
                if ch.is_alphabetic() {
                    piles[i].push(*ch);
                }
            }
        }

        Ok(WarehouseState {
            piles
        })
    }
}

fn day05a() -> String {

    let (state_str, move_str) = include_str!("../input/day05.txt").split_once("\n\n").unwrap();
    let mut state: WarehouseState = state_str.parse().unwrap();

    let move_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for cap in move_re.captures_iter(move_str) {
        let n: usize = cap[1].parse().unwrap();
        let from: usize = cap[2].parse().unwrap();
        let to: usize = cap[3].parse().unwrap();
        state.move_crate(n, from, to);
    }

    state.top_word()
}



// }}}
// day05b {{{

impl WarehouseState {

    fn move_stack(&mut self, n:usize, from:usize, to: usize) {
        let mut buf = Vec::new();

        for _ in 0..n {
            buf.push(self.piles[from-1].pop().unwrap());
        }

        for cr in buf.iter().rev() {
            self.piles[to-1].push(*cr);
        }
    }
}

fn day05b() -> String {

    let (state_str, move_str) = include_str!("../input/day05.txt").split_once("\n\n").unwrap();
    let mut state: WarehouseState = state_str.parse().unwrap();

    let move_re = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

    for cap in move_re.captures_iter(move_str) {
        let n: usize = cap[1].parse().unwrap();
        let from: usize = cap[2].parse().unwrap();
        let to: usize = cap[3].parse().unwrap();
        state.move_stack(n, from, to);
    }

    state.top_word()
}

// }}}
// day06a {{{

fn day06_solve(s: &str, winsize: usize) -> usize {
    s.chars()
     .collect::<Vec<char>>()
     .windows(winsize)
     .enumerate()
     .find(|(_, slice)| slice.iter().unique().count() == winsize)
     .unwrap().0 + winsize
}

fn day06a() -> usize {
    day06_solve(include_str!("../input/day06.txt"), 4)
}

// }}}
// day06b {{{


fn day06b() -> usize {
    day06_solve(include_str!("../input/day06.txt"), 14)
}

// }}}
// day07a {{{

fn day07_parse_tree(input: &str) -> HashMap<String, usize> {

    let mut sizes:HashMap<String, usize> = HashMap::new();
    let mut cd: Vec<&str> = vec!["/"];

    input.lines()
         .for_each(|l| match l.split_whitespace().collect::<Vec<_>>()[..] {
             ["$", "cd", "/"] => {cd = vec!["/"];},
             ["$", "cd", ".."] => {cd.pop();},
             ["$", "cd", d] => {cd.push(d);},
             ["$", "ls"] => {},
             ["dir", _] => {},
             [s, _] => {
                 let s = s.parse::<usize>().unwrap();
                 let l = cd.len();
                 for i in 0..cd.len() {
                     *sizes.entry(cd[0..(l - i)].join("/")).or_insert(0) += s;
                 }
             },
             _ => println!("Unknown line: {:?}", l)
         });
    sizes
}

fn day07a() -> usize {
    let sizes = day07_parse_tree(include_str!("../input/day07.txt"));
    sizes.values().filter(|v| v < &&100000).sum::<usize>()
}

fn day07b() -> usize {
    let sizes = day07_parse_tree(include_str!("../input/day07.txt"));
    let need_to_free = 30000000 - (70000000 - sizes.get("/").unwrap());
    *sizes.values().filter(|v| v > &&need_to_free).min().unwrap()
}


// }}}
