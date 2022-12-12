#![allow(dead_code)]

use std::{collections::HashMap, str::FromStr, collections::HashSet};
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
    aoc_task!(day09a);
    aoc_task!(day09b);
    aoc_task!(day10a);
    aoc_task!(day10b);
    aoc_task!(day11a);
    aoc_task!(day11b);
    aoc_task!(day12a);
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
// day08 TODO {{{

fn day08_parse_rows(input: &str) -> Vec<Vec<usize>> {
    input
        .lines()
        .map(|l| l.chars().map(|ch| ch.to_digit(10).unwrap() as usize).collect())
        .collect()
}


fn day08_row_visibility(row: Vec<usize>) -> Vec<bool> {
    let mut max = -1;

    row.iter()
       .map(|v| {
           if *v as i32 > max {
               max = *v as i32;
               true
           } else {
               false
           }
       }).collect()
}

fn day08a() -> usize {

    let table = day08_parse_rows(include_str!("../input/day08.txt"));
    0
}


// }}}za
// day09 {{{

type Knot = (i32, i32);
type Move = (char, i32);

fn day09_move_tail(head: Knot, tail: Knot) -> Knot {

    let dx = head.0 - tail.0;
    let dy = head.1 - tail.1;

    if dx.abs() >= 2 && dy == 0 {
        (tail.0 + dx.signum(), tail.1)
    } else if dx == 0 && dy.abs() >= 2 {
        (tail.0, tail.1 + dy.signum())
    } else if dx.abs() >= 2 || dy.abs() >= 2 {
        (tail.0 + dx.signum(), tail.1 + dy.signum())
    } else {
        tail
    }
}

fn day09_input() -> Vec<Move> {
    include_str!("../input/day09.txt")
        .lines()
        .map(|l| {
            let (d, n) = l.split_once(' ').unwrap();
            (d.chars().next().unwrap(), n.parse::<i32>().unwrap())
        }).collect()
}

fn day09_solve(n: usize) -> usize {
    let mut knots: Vec<Knot> = (0..n).map(|_| (0, 0)).collect();
    let mut history: Vec<Knot> = vec![knots[0]];

    day09_input().iter().for_each(|m| {
        for _ in 0..m.1 {
            match m.0 {
                'R' => knots[0] = (knots[0].0 + 1, knots[0].1),
                'L' => knots[0] = (knots[0].0 - 1, knots[0].1),
                'U' => knots[0] = (knots[0].0, knots[0].1 + 1),
                'D' => knots[0] = (knots[0].0, knots[0].1 - 1),
                _ => unreachable!()
            }
            for i in 1..n {
                knots[i] = day09_move_tail(knots[i-1], knots[i]);
            }
            history.push(*knots.last().unwrap());
        }
    });
    history.iter().unique().count()
}

fn day09a() -> usize {
    day09_solve(2)
}

fn day09b() -> usize {
    day09_solve(10)
}

// }}}
// day10 {{{


fn day10_input() -> Vec<i32> {

    let mut x: i32 = 1;
    let mut hist_x: Vec<i32> = vec![x];

    include_str!("../input/day10.txt")
        .lines()
        .for_each(|l| match l.split(' ').collect::<Vec<&str>>()[..] {
                    ["noop"] => {
                        hist_x.push(x);
                    },
                    ["addx", v] => {
                        hist_x.push(x);
                        hist_x.push(x);
                        x += v.parse::<i32>().unwrap();
                    }
                    _ => unreachable!()
        });
    hist_x
}

fn day10a() -> i32 {
    day10_input()
        .iter()
        .enumerate()
        .skip(20)
        .step_by(40)
        .map(|(i, &x)| i as i32 * x)
        .sum()
}


fn day10b() -> i32 {

    let mut crt: Vec<char>  = ".".repeat(241).chars().collect();

    day10_input()
        .iter()
        .enumerate()
        .for_each(|(i, &x)| {
            if (x - ((i as i32 - 1) % 40)).abs() <= 1 {
                crt[i] = '#';
            }
        });


    for i in 0..6 {
        println!("{:03} ->  {:} <- {:03}", i*40,  &crt[1+i*40..1+(i+1)*40].iter().collect::<String>(), (i+1)*40);
    }

    0
}



// }}}
// day11 {{{

#[derive(Debug)]
enum Operation {
    Add(i64),
    Mult(i64),
    Power,
}

impl Operation {
    fn apply(&self, v: i64) -> i64 {
        match self {
            Operation::Add(n) => v+n,
            Operation::Mult(n) => v*n,
            Operation::Power => v*v,
        }
    }
}

impl FromStr for Operation {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().collect::<Vec<_>>()[..] {
            ["Operation:", "new", "=", "old", "+", n] => Ok(Operation::Add(n.parse()?)),
            ["Operation:", "new", "=", "old", "*", "old"] => Ok(Operation::Power),
            ["Operation:", "new", "=", "old", "*", n] => Ok(Operation::Mult(n.parse()?)),
            _ => {
                println!("{:?}", s);
                unreachable!();
            }
        }
    }
}


#[derive(Debug)]
enum Condition {
    Divisible(i64),
}

impl Condition {
    fn apply(&self, v: i64) -> bool {
        match self {
            Condition::Divisible(n) => v % n == 0,
        }
    }

    fn value(&self) -> i64 {
        match self {
            Condition::Divisible(n) => *n,
        }
    }
}

impl FromStr for Condition {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.split_whitespace().collect::<Vec<_>>()[..] {
            ["Test:", "divisible", "by", n] => Ok(Condition::Divisible(n.parse()?)),
            _ => unreachable!()
        }
    }
}

#[derive(Debug)]
struct Monkey {
    id: usize,
    items: Vec<i64>,
    operation: Operation,
    condition: Condition,
    if_true_pass_to: usize,
    if_false_pass_to: usize,
    inspected_items: usize
}

// Monkey 0:
//  Starting items: 79, 98
//  Operation: new = old * 19
//  Test: divisible by 23
//    If true: throw to monkey 2
//    If false: throw to monkey 3

impl FromStr for Monkey {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<_> = s.lines().collect();

        let id: usize = lines[0].chars().collect::<Vec<char>>()[7].to_digit(10).unwrap() as usize;
        let items: Vec<i64> = lines[1].split([':', ',']).skip(1)
                                      .map(|v| v[1..].parse().unwrap())
                                      .collect();
        let operation: Operation = lines[2].parse().unwrap();
        let condition: Condition = lines[3].parse().unwrap();

        let if_true: usize = lines[4].split_whitespace().last().unwrap().parse().unwrap();
        let if_false: usize = lines[5].split_whitespace().last().unwrap().parse().unwrap();

        Ok(Monkey {
            id,
            items,
            operation,
            condition,
            if_true_pass_to: if_true,
            if_false_pass_to: if_false,
            inspected_items: 0
        })
    }

}

fn day11_input() -> Vec<Monkey> {
    include_str!("../input/day11.txt")
        .split("\n\n")
        .map(|s| s.parse::<Monkey>().unwrap())
        .collect()
}

fn day11a() -> usize {
    let mut monkeys = day11_input();

    (0..20).for_each(|_| {
        for i in 0..monkeys.len() {

            while let Some(mut v) = monkeys[i].items.pop() {
                let j;
                {
                    let mut m = &mut monkeys[i];
                    m.inspected_items += 1;
                    v = m.operation.apply(v);
                    v = v / 3;

                    if m.condition.apply(v) {
                        j = m.if_true_pass_to;
                    } else {
                        j = m.if_false_pass_to;
                    }
                }
                monkeys[j].items.push(v);
            }

        }
    });

    monkeys.iter().map(|m| m.inspected_items).sorted().rev().take(2).product()
}

fn day11b() -> usize {
    let mut monkeys = day11_input();
    let n: i64 = monkeys.iter().map(|m| m.condition.value()).product();

    (0..10_000).for_each(|_| {
        for i in 0..monkeys.len() {

            while let Some(mut v) = monkeys[i].items.pop() {
                let j;
                {
                    let mut m = &mut monkeys[i];
                    m.inspected_items += 1;
                    v = m.operation.apply(v);
                    v = v % n;

                    if m.condition.apply(v) {
                        j = m.if_true_pass_to;
                    } else {
                        j = m.if_false_pass_to;
                    }
                }
                monkeys[j].items.push(v);
            }

        }
    });

    monkeys.iter().map(|m| m.inspected_items).sorted().rev().take(2).product()
}
// }}}
// day12 {{{

fn ord(ch: char) -> u8 {
    (ch as i32 - 'a' as i32) as u8
}

fn day12_expand(table: Vec<Vec<u8>>, x: (usize, usize)) -> Vec<(usize, usize)> {
    let (i0, j0) = x;
    let mut cnd = Vec::new();

    if i0 + 1 < table.len() {
        cnd.push((i0+1, j0));
    }

    if i0 > 0 {
        cnd.push((i0-1, j0));
    }

    if j0 + 1 < table[0].len() {
        cnd.push((i0, j0+1));
    }

    if j0 > 0 {
        cnd.push((i0, j0-1));
    }

    cnd = cnd.into_iter().filter(|(i, j)| {
        table[*i][*j] - 1 <= table[i0][j0]
    }).collect();

    cnd
}

fn day12_bfs(table: Vec<Vec<u8>>, start: (usize, usize), goal: (usize, usize)) -> u32 {

    let mut visited = HashSet::new();
    let mut parrent = HashMap::new();
    let mut queue = vec![start];

    visited.insert(start);

    while queue.len() > 0 {



    }



    1
}

fn day12a() -> usize {

    let mut start: (usize, usize) = (0, 0);
    let mut goal: (usize, usize) = (0, 0);
    let table: Vec<Vec<u8>>  = include_str!("../input/day12_test.txt")
        .lines()
        .enumerate()
        .map(|(i, l)| {
            l.chars().enumerate().map(|(j, ch)| {
                if ch == 'S' {
                    start = (i, j);
                    return ord('a');
                }
                if ch == 'E' {
                    goal = (i, j);
                    return ord('z');
                }
                ord(ch)
            }).collect()
        }).collect();




    println!("{:?}", table);
    1
}




// }}}
