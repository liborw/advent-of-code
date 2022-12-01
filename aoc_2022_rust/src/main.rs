#![allow(dead_code)]

use took::took;

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
