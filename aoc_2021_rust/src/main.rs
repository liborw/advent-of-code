#![allow(dead_code)]

use took::took;

macro_rules! aoc_task {
    ($f:ident) => {

        let (took, result) = took($f);
        println!("{} took: {} result: {}", stringify!($f), took, result);
    };
}

fn main() {
    aoc_task!(day01a);
    aoc_task!(day01b);
    aoc_task!(day02a);
    aoc_task!(day02b);
    aoc_task!(day03a);
    aoc_task!(day03b);
}

// Day01a {{{

fn day01a() -> usize {
    include_str!("../input/day01a.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<i16>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count()
}

// }}}

fn day01b() -> usize {
    include_str!("../input/day01b.txt")
        .lines()
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u16>>()
        .windows(3)
        .map(|w| w[0] + w[1] + w[2])
        .collect::<Vec<u16>>()
        .windows(2)
        .filter(|w| w[0] < w[1])
        .count()
}

fn day02a() -> i32 {
   let (h ,v) = include_str!("../input/day02a.txt")
                    .lines()
                    .map(|l| l.split_once(" ").unwrap())
                    .fold((0, 0), |(h, v), (t, k)| {
                        match (t, k.parse::<i32>().unwrap() ){
                            ("forward", k) => (h, v + k),
                            ("down", k) => (h + k, v),
                            ("up", k) => (h - k, v),
                            _ => unreachable!()
                        }
                    });

   h * v
}

fn day02b() -> i32 {

   let (_, h ,v) = include_str!("../input/day02b.txt")
                    .lines()
                    .map(|l| l.split_once(" ").unwrap())
                    .fold((0, 0, 0), |(a, h, v), (t, k)| {
                        match (t, k.parse::<i32>().unwrap() ){
                            ("forward", k) => (a, h + a * k, v + k),
                            ("down", k) => (a + k, h, v),
                            ("up", k) => (a - k, h, v),
                            _ => unreachable!()
                        }
                    });
   h * v
}

fn element_wise_add(a:Vec<u32>, b: Vec<u32>) -> Vec<u32> {
    a.iter().zip(b.iter()).map(|(a, b)| a + b).collect()
}

fn bool_to_int(b: bool) -> u32 {
    match b {
        true => 1,
        false => 0,
    }
}

fn fold_binary_vec(v: Vec<u32>) -> u32 {
    v.iter().fold(0, |acc, digit| {(acc << 1) + digit})
}

fn day03a() -> u32 {
    let (n, vec) = include_str!("../input/day03a.txt")
        .lines()
        .map(|l| l.chars().map(|v| v.to_digit(2).unwrap()).collect::<Vec<u32>>())
        .enumerate()
        .reduce(|(_, acum), (i, vec)| (i, element_wise_add(acum, vec))).unwrap();

    let n: u32 = ((n as u32) + 1) / 2;
    let gamma = fold_binary_vec(vec.iter().map(|v| bool_to_int(v > &n)).collect());
    let epsilon = fold_binary_vec(vec.iter().map(|v| bool_to_int(v < &n)).collect());
    gamma * epsilon
}

// Day 03 B


fn day03b() -> u32 {

    let report:Vec<_> = include_str!("../input/day03b.txt")
        .lines()
        .map(|l| l.chars().map(|v| v.to_digit(2).unwrap() as u32).collect::<Vec<u32>>())
        .collect();

    let width:usize = report[0].len();

    let oxy = (0..width).scan(report.clone(), |rep, i| {
                            let one: bool = rep.iter().filter(|v| v[i] == 1).count() >= (rep.len() + 1) / 2;
                            rep.retain(|v| (v[i] == 1) != one);
                            rep.first().cloned()
                        }).last().unwrap();
    let oxy = fold_binary_vec(oxy);

    let co2 = (0..width).scan(report.clone(), |rep, i| {
                            let one: bool = rep.iter().filter(|v| v[i] == 1).count() >= (rep.len() + 1) / 2;
                            rep.retain(|v| (v[i] == 1) == one);
                            rep.first().cloned()
                        }).last().unwrap();
    let co2= fold_binary_vec(co2);
    oxy * co2
}


