#![allow(dead_code)]

use std::{collections::HashMap, str::FromStr, collections::{HashSet, VecDeque}, fmt::Display, ops};
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
    aoc_task!(day12b);
    aoc_task!(day13a);
    aoc_task!(day13b);
    aoc_task!(day15a);
    aoc_task!(day15b);
    aoc_task!(day16a);
    //aoc_task!(day16b);
    aoc_task!(day17a);
    aoc_task!(day17b);
    aoc_task!(day18a);
    aoc_task!(day18b);
    aoc_task!(day19a);
    aoc_task!(day19b);
}

// }}}
// utils {{{

#[derive(Debug)]
struct NameMap
{
    names: HashMap<String, usize>,
    index: usize
}

impl NameMap {

    fn new() -> Self {
        NameMap {
            names: HashMap::new(),
            index: 0
        }
    }

    fn get(&mut self, name: String) -> usize {
        if !self.names.contains_key(&name) {
            self.names.insert(name.clone(), self.index);
            self.index += 1;
        }

        *self.names.get(&name).unwrap()
    }
}

impl<const N: usize> From<[&str; N]> for NameMap {

    fn from(arr: [&str; N]) -> Self {
        let mut map = NameMap::new();
        for v in arr {
            map.get(v.to_string());
        }
        map
    }
}

#[derive(Debug)]
struct BoundingBox<T>{
    xmin: T,
    ymin: T,
    xmax: T,
    ymax: T
}

impl BoundingBox<i32> {

    fn new(xmin:i32, ymin:i32, xmax:i32, ymax:i32) -> Self {
        BoundingBox { xmin, ymin, xmax, ymax }
    }

    fn zero() -> Self {
        BoundingBox {xmin: i32::MAX, ymin: i32::MAX, xmax: i32::MIN, ymax: i32::MIN }
    }

    fn push(&mut self, (x, y): (i32, i32)) -> () {
        if self.xmin > x {self.xmin = x};
        if self.xmax < x {self.xmax = x};
        if self.ymin > y {self.ymin = y};
        if self.ymax < y {self.ymax = y};
    }
}

type SparseMap<T> = HashMap<(i32, i32), T>;


trait Map<T> {
    fn bb(&self) -> BoundingBox<i32>;
    fn print(&self, default: char) -> ();
    fn print_fliptb(&self, default: char) -> ();
    fn print_bb(&self, bb: &BoundingBox<i32>, default: char) -> ();
    fn print_bb_fliptb(&self, bb: &BoundingBox<i32>, default: char) -> ();
}

impl<T: Display> Map<T> for SparseMap<T> {
    fn bb(&self) -> BoundingBox<i32> {
        let mut bb = BoundingBox::zero();
        for p in self.keys() {
            bb.push(*p);
        }
        bb
    }

    fn print(&self, default: char) -> () {
        let bb = self.bb();
        self.print_bb(&bb, default);
    }

    fn print_fliptb(&self, default: char) -> () {

        let bb = self.bb();
        self.print_bb_fliptb(&bb, default);

    }

    fn print_bb(&self, bb: &BoundingBox<i32>, default: char) -> () {
        for y in bb.ymin-1..=bb.ymax+1 {
            for x in bb.xmin-1..=bb.xmax+1 {
                match self.get(&(x,y)) {
                    Some(v) => print!("{}", v),
                    None => print!("{}", default)
                }
            }
            println!();
        }
    }

    fn print_bb_fliptb(&self, bb: &BoundingBox<i32>, default: char) -> () {
        for y in 0..=bb.ymax - bb.ymin {
            for x in bb.xmin-1..=bb.xmax+1 {
                match self.get(&(x,bb.ymax - y)) {
                    Some(v) => print!("{}", v),
                    None => print!("{}", default)
                }
            }
            println!();
        }
    }
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

fn ord(ch: char) -> i8 {
    (ch as i32 - 'a' as i32) as i8
}

fn day12_expand(table: &Vec<Vec<i8>>, x: (usize, usize)) -> Vec<(usize, usize)> {
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

type Pos = (usize, usize);
type Table = Vec<Vec<i8>>;


fn day12_bfs<FNA, FNB>(table: &Table,
             start: Pos,
             expand: FNA,
             is_goal: FNB) -> Option<u32>
    where FNA: Fn(&Table, Pos) -> Vec<Pos>,
          FNB: Fn(Pos) -> bool

{

    let mut visited = HashSet::new();
    let mut parrent = HashMap::new();
    let mut queue = VecDeque::new();

    visited.insert(start);
    queue.push_back(start);

    while queue.len() > 0 {
        let mut n = queue.pop_front().unwrap();

        if is_goal(n) {

            let mut i = 0;
            while let Some(&prev) = parrent.get(&n) {
                n = prev;
                i += 1;
            }
            return Some(i);
        }

        for next in expand(&table, n).into_iter() {
            if visited.insert(next) {
                parrent.insert(next, n);
                queue.push_back(next);
            }
        }
    }
    None
}

fn day12a() -> u32 {

    let mut start: Pos = (0, 0);
    let mut goal: Pos = (0, 0);
    let table: Vec<Vec<i8>>  = include_str!("../input/day12.txt")
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



    day12_bfs(&table, start, day12_expand, |p| p == goal ).unwrap()
}

fn day12b_expand(table: &Vec<Vec<i8>>, x: (usize, usize)) -> Vec<(usize, usize)> {
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
        table[*i][*j] >= table[i0][j0] - 1
    }).collect();

    cnd
}


fn day12b() -> u32 {

    let mut start: Pos = (0, 0);
    let mut goal: Pos = (0, 0);
    let table: Vec<Vec<i8>>  = include_str!("../input/day12.txt")
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



    day12_bfs(&table, goal, day12b_expand, |(i,j)| table[i][j] == 0 ).unwrap()
}



// }}}
// day13 {{{

use std::cmp::Ordering;

#[derive(Debug, Clone, PartialEq, Eq)]
enum NestedList {
    Empty,
    Value(i32),
    List(Vec<NestedList>),
}

impl FromStr for NestedList {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use NestedList::*;

        let tokens = Regex::new(r"[\[\]]|\d+").unwrap();
        let mut stack = vec![];
        let mut list = Empty;

        for token in tokens.captures_iter(&s) {
            match (&token[0], &mut list) {
                ("[", Empty) => {
                    list = List(vec![]);
                }
                ("[", List(_)) => {
                    stack.push(list);
                    list = List(vec![]);
                }
                ("[", _) => unreachable!(),
                ("]", List(_)) => {
                    if let Some(mut nl) = stack.pop() {
                        if let List(li) = &mut nl {
                            li.push(list);
                            list = nl;
                        }
                    } else {
                        return Ok(list);
                    }
                }
                ("]", _) => unreachable!(),
                (num, List(li)) => {
                    li.push(Value(num.parse().unwrap()));
                }
                _ => unreachable!()
            }
        }
        Ok(Empty)
    }
}

impl PartialOrd for NestedList {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        use NestedList::*;
        use Ordering::*;
        match (self, other) {
            (Empty, Empty) => Some(Equal),
            (Empty, _) => Some(Less),
            (_, Empty) => Some(Greater),
            (Value(v1), Value(v2)) => v1.partial_cmp(v2),
            (List(li), Value(v)) => li.partial_cmp(&vec![Value(*v)]),
            (Value(v), List(li)) => vec![Value(*v)].partial_cmp(li),
            (List(li1), List(li2)) => li1.partial_cmp(li2),
        }
    }
}

impl Display for NestedList {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use NestedList::*;
        match self {
            Value(v) => v.fmt(f),
            Empty => write!(f, "[]"),
            List(li) => {

                if li.is_empty() {
                    return write!(f, "[]");
                }

                let mut str = String::new();
                str.push_str(&"[".to_string());
                for v in &li[0..li.len()-1] {
                    str.push_str(&v.to_string());
                    str.push_str(&", ".to_string());
                }

                str.push_str(&li[li.len()-1].to_string());
                str.push_str(&"]".to_string());
                write!(f, "{}", str)
            }
        }
    }
}


impl Ord for NestedList {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

fn day13_input() -> Vec<NestedList>  {
    include_str!("../input/day13.txt")
        .lines()
        .filter(|l| !l.is_empty())
        .map(|l| l.parse().unwrap())
        .collect()
}

fn day13a() -> i32 {
    day13_input()[..]
        .chunks(2)
        .enumerate()
        .map(|(i, chunk)| {
            if chunk[0] <= chunk[1] {
                i as i32 + 1
            } else {
                0
            }
        }).sum()
}

fn day13b() -> i32 {
    let mut input = day13_input();
    let divider2: NestedList = "[[2]]".parse().unwrap();
    let divider6: NestedList = "[[6]]".parse().unwrap();
    input.push(divider2.clone());
    input.push(divider6.clone());
    input.sort();

    input.iter()
         .enumerate()
         .fold(1, |acc, (i, v)| {
             if (v == &divider2) | (v == &divider6) {
                 acc * (i as i32 + 1)
             } else {
                 acc
             }
         })
}

// }}}
// day14 {{{

type Loc = (i32, i32);

struct CaveMap {
    y_max: i32,
    map: HashMap<Loc, char>,
}

impl CaveMap {

    fn bb(&self) -> (i32, i32, i32, i32) {
        let (mut x0, mut y0, mut x1, mut y1) = (i32::MAX, i32::MAX, i32::MIN, i32::MIN);

        for (xi, yi) in self.map.keys() {
            if x0 > *xi {x0 = *xi};
            if x1 < *xi {x1 = *xi};
            if y0 > *yi {y0 = *yi};
            if y1 > *yi {y1 = *yi};
        }
        (x0, y0, x1, y1)
    }

}

// }}}
// day15 {{{

#[derive(Debug)]
struct Sensor {
    x: i32,
    y: i32,
    r: i32
}

impl Sensor {

    fn new(sx: i32, sy: i32, bx: i32, by: i32) -> Self {

        Sensor{
            x: sx,
            y: sy,
            r: (sx - bx).abs() + (sy - by).abs()
        }
    }

    fn in_range(&self, x: i32, y: i32) -> bool {
        (self.x - x).abs() + (self.y - y).abs() <= self.r
    }

}

fn day15_input() -> Vec<Sensor> {
    let tokens = Regex::new(r"(-?\d+)").unwrap();
    include_str!("../input/day15.txt")
        .lines()
        .map(|l| {

            let v: Vec<i32> = tokens
                .captures_iter(l)
                .map(|c| c[1].parse().unwrap())
                .collect();

            let x = v[0];
            let y = v[1];
            let bx = v[2];
            let by = v[3];
            let r = (x - bx).abs() + (y - by).abs();

            Sensor{x, y, r}
        }).collect()
}

fn day15_ranges(sb: &Vec<Sensor>, y0: i32) -> Vec<(i32,i32)> {
    sb.iter()
        .map(|s| {
            let x0 = s.x - (s.r - (s.y - y0).abs());
            let x1 = s.x + (s.r - (s.y - y0).abs());
            (x0, x1)
        })
        .filter(|(x0, x1)| x0 < x1)
        .sorted_by_key(|(x0, _)| *x0)
        .collect()
}

fn day15a() -> i32 {
    let sensors = day15_input();
    let y0 = 2_000_000;
    let ranges = day15_ranges(&sensors, y0);

    let mut ret = 0;
    let mut range = ranges[0];
    for i in 1..ranges.len() {
        if ranges[i].0 <= range.1 {
            if ranges[i].1 > range.1 {
                range.1 = ranges[i].1;
            }
        } else {
            ret += (range.0 - range.1).abs();
            range = ranges[i];
        }
    }
    ret += (range.0 - range.1).abs();
    ret
}

fn day15b() -> i64 {

    let sensors = day15_input();

    for y0 in 0..4_000_000 {
        let ranges = day15_ranges(&sensors, y0);

        let mut range = ranges[0];
        for i in 1..ranges.len() {
            if ranges[i].0 <= range.1 {
                if ranges[i].1 > range.1 {
                    range.1 = ranges[i].1;
                }
            } else {

                return (range.1 + 1) as i64 * 4_000_000 + y0 as i64;
            }
        }
    }
    0
}



// }}}
// day16 {{{

use ndarray::Array2;

#[derive(Debug, Clone)]
struct Valve {
    id: usize,
    next: Vec<usize>,
    flow: i32
}

fn day16_input() -> Vec<Valve> {
    let mut nmap: NameMap = ["AA"].into();
    let tokens = Regex::new(r"(\d+|[A-Z]{2})").unwrap();
    include_str!("../input/day16.txt")
        .lines()
        .map(|l| {
            let capture: Vec<_> = tokens.captures_iter(l).collect();
            let mut next = vec![];
            for i in 2..capture.len() {
                next.push(nmap.get(capture[i][1].to_string()));
            }

            Valve{
                id: nmap.get(capture[0][1].to_string()),
                flow: capture[1][1].parse().unwrap(),
                next
            }

        })
        .sorted_by_key(|v| v.id)
        .collect()
}

fn day16_warshall(valves: &Vec<Valve>) -> Array2<i32> {
    let n = valves.len();
    let mut dist = Array2::ones((n, n)) * i32::MAX;

    for v in valves {
        dist[[v.id, v.id]] = 0;
        for id in v.next.iter() {
            dist[[v.id, *id]] = 1;
        }
    }

    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                if dist[(i,j)] as i64 > dist[(i,k)] as i64 + dist[(k,j)] as i64 {
                    dist[(i,j)] = dist[(i,k)] + dist[(k,j)];
                }
            }
        }
    }
    dist
}

fn day16a_DFS(start: usize, open:Vec<bool>, flow: i32, ttl: i32, valves: &Vec<Valve>, dist: &Array2<i32>) -> i32 {

    if ttl < 0 {
        return 0;
    }

    if ttl == 0 {
        return flow;
    }

    let current_flow: i32 = open.iter().zip(valves.iter()).map(|(&o, v)| o as i32 * v.flow).sum();

    let mut a_flow = flow + current_flow * ttl;
    for v in valves {
        if (v.flow > 0) & !open[v.id] {

            let d_t = dist[[start, v.id]] + 1;
            let d_flow = flow + d_t * current_flow;
            let mut d_open = open.clone();
            d_open[v.id] = true;

            let f = day16a_DFS(v.id, d_open, d_flow, ttl - d_t, valves, dist);
            if a_flow < f {
                a_flow = f
            }
        }
    }
    a_flow
}

fn day16a() -> i32 {
    let valves = day16_input();
    let n = valves.len();
    let dist = day16_warshall(&valves);

    // DFS
    day16a_DFS(0, vec![false; n],0, 30, &valves, &dist)
}

fn day16b() -> i32 {
    let valves = day16_input();
    let n = valves.len();
    let dist = day16_warshall(&valves);

    // DFS
    day16a_DFS(0, vec![false; n],0, 24*2, &valves, &dist)
}


// }}}
// day17 {{{

#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T
}

impl<T> Point<T> {

    fn new(x: T, y: T) -> Point<T> {
        Point{x, y}
    }
}

impl<T> From<(T,T)> for Point<T> {

    fn from(p: (T,T)) -> Self {
        Point::new(p.0, p.1)
    }

}

#[derive(Debug, PartialEq)]
enum Rock {
    VLine,
    Plus,
    InvertedL,
    Hline,
    Square
}

impl Rock {

    fn start(&self, top: &i32) -> Point<i32> {
        use Rock::*;
        match self {
            VLine => Point::new(2, top + 4),
            Plus => Point::new(2, top + 5),
            InvertedL => Point::new(2, top + 4),
            Hline => Point::new(2, top + 4),
            Square => Point::new(2, top + 4)
        }
    }

    fn points(&self, p: &Point<i32>) -> Vec<Point<i32>> {
        use Rock::*;
        match self {
            VLine => vec![
                Point::new(p.x, p.y),
                Point::new(p.x + 1, p.y),
                Point::new(p.x + 2, p.y),
                Point::new(p.x + 3, p.y)
            ],
            Plus => vec![
                Point::new(p.x, p.y),
                Point::new(p.x + 1, p.y),
                Point::new(p.x + 1, p.y + 1),
                Point::new(p.x + 1, p.y - 1),
                Point::new(p.x + 2, p.y),
            ],
            InvertedL => vec![
                Point::new(p.x, p.y),
                Point::new(p.x + 1, p.y),
                Point::new(p.x + 2, p.y),
                Point::new(p.x + 2, p.y + 1),
                Point::new(p.x + 2, p.y + 2),
            ],
            Hline => vec![
                Point::new(p.x, p.y),
                Point::new(p.x, p.y + 1),
                Point::new(p.x, p.y + 2),
                Point::new(p.x, p.y + 3)
            ],
            Square => vec![
                Point::new(p.x, p.y),
                Point::new(p.x + 1, p.y),
                Point::new(p.x, p.y + 1),
                Point::new(p.x + 1, p.y + 1)
            ],
        }

    }

    fn test(&self, pos: &Point<i32>, map: &SparseMap<char>, width: &usize) -> bool {

        if pos.x < 0 {
            return false;
        }

        for p in self.points(pos) {
            if map.contains_key(&(p.x, p.y)) {
                return false;
            }

            if p.x >= *width as i32 {
                return false;
            }

            if p.y <= 0 {
                return false;
            }
        }

        true
    }

    fn place(&self, pos: &Point<i32>, map: &mut SparseMap<char>) -> i32 {
        use Rock::*;
        let mut y_max = 0;

        let ch = match self {
            VLine => 'V',
            Plus => 'P',
            InvertedL => 'I',
            Hline => 'H',
            Square => 'S'
        };

        for p in self.points(pos) {
            map.insert((p.x, p.y), ch);
            if y_max < p.y { y_max = p.y }
        }
        y_max
    }
}

fn day17_input() -> Vec<i32> {
    include_str!("../input/day17_test.txt")
        .chars()
        .filter_map(|ch| {
            match ch {
                '<' => Some(-1),
                '>' => Some(1),
                _   => None
            }
        }).collect()
}


fn day17_solve(rskip: usize, fskip: usize, take: usize) -> i32 {
    use Rock::*;
    let mut top = 0;
    let width = 7;
    let mut map = SparseMap::new();
    let mut jets = day17_input().into_iter().cycle().skip(fskip);

    for rock in vec![VLine, Plus, InvertedL, Hline, Square].iter().cycle().skip(rskip).take(take) {
        let mut p = rock.start(&top);
        loop {
            let jet = jets.next().unwrap();

            let mut new_p = Point::new(p.x + jet, p.y);
            if rock.test(&new_p, &map, &width) {
                p = new_p;
            }

            new_p = Point::new(p.x, p.y - 1);
            if rock.test(&new_p, &map, &width) {
                p = new_p;
            } else {
                let new_top = rock.place(&p, &mut map);
                if top < new_top {top = new_top.clone()};
                break;
            }
        }
    }

    top
}

fn day17_find_repeat(jets: Vec<i32>, take: usize) -> Option<(usize, usize, usize, usize, usize, usize)> {
    use Rock::*;
    let mut top = 0;
    let width = 7;
    let mut map = SparseMap::new();

    let mut jets_cycle = jets.iter().cycle();

    let mut patters: HashMap<(i32, i32, String), (i32, i32)> = HashMap::new();

    let mut rock_i = 0;
    let mut jet_i = 0;
    for rock in vec![VLine, Plus, InvertedL, Hline, Square].iter().cycle().take(take) {
        rock_i += 1;
        let mut p = rock.start(&top);

        let margin = 20;
        let mut str_vec = Vec::new();
        for y in top-margin..=top {
            for x in 0..7 {
                str_vec.push(*map.get(&(x,y)).unwrap_or(&'.'));
            }
        }

        let pat = ((rock_i % 5) as i32, (jet_i % jets.len()) as i32, str_vec.iter().collect());
        if patters.contains_key(&pat) {
            let (l0, r0) = patters.get(&pat).unwrap();
            let li = top - l0;
            let ri = rock_i - 1 - r0;
            let rs = rock_i % 5;
            let fs = jet_i % jets.len();
            return Some((*l0 as usize, *r0 as usize, li as usize, ri as usize, rs as usize, fs as usize))
        } else {
            patters.insert(pat, (top, rock_i-1));
        }


        loop {
            let jet = jets_cycle.next().unwrap();
            jet_i += 1;


            let mut new_p = Point::new(p.x + jet, p.y);
            if rock.test(&new_p, &map, &width) {
                p = new_p;
            }

            new_p = Point::new(p.x, p.y - 1);
            if rock.test(&new_p, &map, &width) {
                p = new_p;
            } else {
                let new_top = rock.place(&p, &mut map);
                if top < new_top {top = new_top.clone()};
                break;
            }
        }
    }

    None
}

fn day17a() -> i32 {
    day17_solve(0, 0, 50)
}

fn day17b() -> u64 {
    let jets = day17_input();
    let t: usize = 1_000_000_000_000;
    //let t: usize = 2022;
    let (l0, r0, li, ri, rs, fs) = day17_find_repeat(jets, 10000).unwrap();
    let n = (t - r0) / ri;
    l0 as u64 + (n as u64 * li as u64) as u64 + day17_solve(rs, fs, (t - r0) % ri) as u64
}

// }}}
// day18: {{{

fn day18_input() -> HashSet<(i32, i32, i32)> {

    let input: Vec<Vec<i32>> = include_str!("../input/day18.txt")
        .lines()
        .map(|l| l.split(",").map(|v| v.parse().unwrap()).collect())
        .collect();

    let mut set = HashSet::new();
    for line in input.iter() {
        set.insert((line[0], line[1], line[2]));
    }
    set
}

fn day18a() -> usize {
    let set = day18_input();

    let mut cnt: usize = 0;
    for &(x,y,z) in set.iter() {
        if !set.contains(&(x+1,y,z)) {cnt += 1};
        if !set.contains(&(x-1,y,z)) {cnt += 1};
        if !set.contains(&(x,y+1,z)) {cnt += 1};
        if !set.contains(&(x,y-1,z)) {cnt += 1};
        if !set.contains(&(x,y,z+1)) {cnt += 1};
        if !set.contains(&(x,y,z-1)) {cnt += 1};
    }
    cnt
}



fn day18b() -> usize {

    let set = day18_input();
    let mut min = (i32::MAX, i32::MAX, i32::MAX);
    let mut max = (i32::MIN, i32::MIN, i32::MIN);

    for &(x,y,z) in set.iter() {
        if min.0 > x {min.0 = x};
        if min.1 > y {min.1 = y};
        if min.2 > z {min.2 = z};
        if max.0 < x {max.0 = x};
        if max.1 < y {max.1 = y};
        if max.2 < z {max.2 = z};
    }

    let mut outside: HashSet<(i32,i32,i32)> = HashSet::new();
    let mut queue: VecDeque<(i32,i32,i32)> = VecDeque::new();
    let start = (min.0 - 1, min.1 - 1, min.2 - 1);

    queue.push_front(start);
    outside.insert(start);
    while !queue.is_empty() {
        let n = queue.pop_back().unwrap();

        let new = (n.0+1, n.1, n.2);
        if (max.0 + 1 >= new.0) & !outside.contains(&new) & !set.contains(&new) {
            queue.push_front(new);
            outside.insert(new);
        };

        let new = (n.0-1, n.1, n.2);
        if (min.0 - 1 <= new.0) & !outside.contains(&new) & !set.contains(&new) {
            queue.push_front(new);
            outside.insert(new);
        };

        let new = (n.0, n.1+1, n.2);
        if (max.1 + 1 >= new.1) & !outside.contains(&new) & !set.contains(&new) {
            queue.push_front(new);
            outside.insert(new);
        };

        let new = (n.0, n.1-1, n.2);
        if (min.1 - 1 <= new.1) & !outside.contains(&new) & !set.contains(&new) {
            queue.push_front(new);
            outside.insert(new);
        };

        let new = (n.0, n.1, n.2+1);
        if (max.2 + 1 >= new.2) & !outside.contains(&new) & !set.contains(&new) {
            queue.push_front(new);
            outside.insert(new);
        };

        let new = (n.0, n.1, n.2-1);
        if (min.2 - 1 <= new.2) & !outside.contains(&new) & !set.contains(&new) {
            queue.push_front(new);
            outside.insert(new);
        };
    }

    let mut cnt: usize = 0;
    for &(x,y,z) in set.iter() {
        if outside.contains(&(x+1,y,z)) {cnt += 1};
        if outside.contains(&(x-1,y,z)) {cnt += 1};
        if outside.contains(&(x,y+1,z)) {cnt += 1};
        if outside.contains(&(x,y-1,z)) {cnt += 1};
        if outside.contains(&(x,y,z+1)) {cnt += 1};
        if outside.contains(&(x,y,z-1)) {cnt += 1};
    }

    cnt
}

// }}}
// day19 {{{

#[derive(Debug, PartialEq, Eq, Clone, Hash)]
struct Resources {
    ore: i32,
    clay: i32,
    obsidian: i32,
    geode: i32,
    ore_r: i32,
    clay_r: i32,
    obsidian_r: i32,
    geode_r: i32
}

impl Resources {

    fn new(ore: i32, clay: i32, obsidian: i32, geode: i32, ore_r: i32, clay_r: i32, obsidian_r: i32, geode_r: i32) -> Resources {
        Resources{ore, clay, obsidian, geode, ore_r, clay_r, obsidian_r, geode_r}
    }

    fn zero() -> Resources {
        Resources{ore: 0, clay: 0, obsidian: 0, geode: 0, ore_r: 0, clay_r: 0, obsidian_r: 0, geode_r: 0}
    }

    fn test(&self, action: &Self) -> bool {
        ((self.ore + action.ore) >= 0) &
        ((self.clay + action.clay) >= 0) &
        ((self.obsidian + action.obsidian) >= 0) &
        ((self.geode + action.geode) >= 0) &
        ((self.ore_r + action.ore_r) >= 0) &
        ((self.clay_r + action.clay_r) >= 0) &
        ((self.obsidian_r + action.obsidian_r) >= 0) &
        ((self.geode_r + action.geode_r) >= 0)
    }

    fn step(&self) -> Resources {
        Resources {
            ore: self.ore + self.ore_r,
            clay: self.clay + self.clay_r,
            obsidian: self.obsidian + self.obsidian_r,
            geode: self.geode + self.geode_r,
            ore_r: self.ore_r,
            clay_r: self.clay_r,
            obsidian_r: self.obsidian_r,
            geode_r: self.geode_r
        }
    }

    fn has_more_robots(&self, other: &Self) -> bool {
        (self.ore_r <= other.ore_r) &
        (self.clay_r <= other.clay_r)
        //(self.obsidian_r <= other.obsidian_r)
    }
}

impl ops::Add<Resources> for Resources {
    type Output = Resources;

    fn add(self, rhs: Resources) -> Self::Output {
        Resources {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
            ore_r: self.ore_r + rhs.ore_r,
            clay_r: self.clay_r + rhs.clay_r,
            obsidian_r: self.obsidian_r + rhs.obsidian_r,
            geode_r: self.geode_r + rhs.geode_r
        }
    }
}

impl PartialOrd for Resources {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.geode.partial_cmp(&other.geode)
    }
}

impl Ord for Resources {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl Display for Resources {

    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "[{} {} {} {} {} {} {} {}]", self.ore, self.clay, self.obsidian, self.geode, self.ore_r, self.clay_r, self.obsidian_r, self.geode_r)
    }
}

fn day19_input() -> Vec<Vec<Resources>> {
    let num_rs = Regex::new(r"\d+").unwrap();
    let mut blueprints = Vec::new();
    for l in include_str!("../input/day19.txt").lines() {
        let mut blueprint = Vec::new();

        let m: Vec<_> = num_rs.captures_iter(l).collect();

        let ore: i32 = m[1][0].parse().unwrap();
        blueprint.push(Resources::new(-ore,0,0,0,1,0,0,0));

        let ore: i32 = m[2][0].parse().unwrap();
        blueprint.push(Resources::new(-ore,0,0,0,0,1,0,0));

        let ore: i32 = m[3][0].parse().unwrap();
        let clay: i32 = m[4][0].parse().unwrap();
        blueprint.push(Resources::new(-ore,-clay,0,0,0,0,1,0));

        let ore: i32 = m[5][0].parse().unwrap();
        let obsidian: i32 = m[6][0].parse().unwrap();
        blueprint.push(Resources::new(-ore,0,-obsidian,0,0,0,0,1));

        blueprint.push(Resources::new(0,0,0,0,0,0,0,0));

        blueprints.push(blueprint);
    }
    blueprints
}

fn day19_bfs(blueprint: &Vec<Resources>, start: &Resources, ttl: i32) -> i32 {

    let mut queue = VecDeque::new();
    let mut seen = HashSet::new();
    let mut best_geode = 0;

    queue.push_back((ttl, start.clone()));

    let mut max = Resources::zero();
    for a in blueprint {
        if max.ore_r < -a.ore {
            max.ore_r = -a.ore + 1
        }

        if max.clay_r < -a.clay {
            max.clay_r = -a.clay + 1
        }

        if max.obsidian_r < -a.obsidian {
            max.obsidian_r = -a.obsidian + 1
        }
    }


    println!("max: {}", max);

    while !queue.is_empty() {
        let (ttl, res) = queue.pop_front().unwrap();
        // println!("{} {}", ttl, res);

        if res.geode > best_geode {
            best_geode = res.geode;
        }

        if ttl > 0 {
            let mut can_build_robot = false;
            for (i, a) in blueprint.iter().enumerate() {
                if res.test(a) & ((i < 4) | !can_build_robot) {

                    let mut new_res = res.step();
                    new_res = new_res + a.clone();

                    if !new_res.has_more_robots(&max) {
                        continue
                    }

                    if !seen.contains(&new_res) {
                        seen.insert(new_res.clone());
                        queue.push_back((ttl - 1, new_res));
                        can_build_robot = false;
                    }
                }
            }
        }
    }
    best_geode
}

fn day19a() -> i32 {
    let blueprints = day19_input();

    let resources = Resources::new(0,0,0,0,1,0,0,0);
    blueprints.iter().enumerate().map(|(i, b)| (i as i32 + 1) * day19_bfs(b, &resources, 24)).sum()
}

fn day19b() -> i32 {
    let blueprints = day19_input();

    let resources = Resources::new(0,0,0,0,1,0,0,0);
    blueprints.iter().take(3).map(|b| day19_bfs(b, &resources, 32)).product()
}


// }}}