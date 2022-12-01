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
    let max_calories = include_str!("../input/day01.txt")
        .lines()
        .fold(vec![0], |mut acc, l| {
            match l.parse::<i32>() {
                Ok(n) => {
                    let i = acc.len() - 1;
                    acc[i] += n;
                }
                Err(_) => acc.push(0),
            }
            acc
        })
        .iter()
        .max()
        .unwrap()
        .to_owned();

    max_calories
}

// }}}
// day01b {{{

fn day01b() -> i32 {
    let mut elven_calories =
        include_str!("../input/day01.txt")
            .lines()
            .fold(vec![0], |mut acc, l| {
                match l.parse::<i32>() {
                    Ok(n) => {
                        let i = acc.len() - 1;
                        acc[i] += n;
                    }
                    Err(_) => acc.push(0),
                }
                acc
            });

    elven_calories.sort();
    elven_calories.reverse();

    let total_of_top_three = elven_calories.iter().take(3).sum();
    total_of_top_three
}

// }}}
