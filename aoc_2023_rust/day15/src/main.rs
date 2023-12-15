use std::collections::HashMap;

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

fn parse(input: &str) -> Vec<&str> {
    input.strip_suffix("\n").unwrap().split(',').collect()
}

fn hash(input: &[u8]) -> usize {
    input.iter().fold(0, |acc, &v| {
        ((acc + v as usize)*17) % 256
    })
}

fn part1(input: &str) -> usize {
    parse(input).into_iter().map(|s| {
        let v = hash(s.as_bytes());
        println!("{s:?} => {v}");
        v
    }).sum()
}

fn split_instr(instr: &str) -> (&str, char, Option<usize>) {
    if instr.contains("=") {
        let (label, f) = instr.split_once("=").unwrap();
        (label, '=', Some(f.parse::<usize>().unwrap()))
    } else {
        let (label, _) = instr.split_once("-").unwrap();
        (label, '-', None)
    }
}

fn print_boxes(boxes: &Vec<Vec<(&str, usize)>>) -> () {
    for (i, b) in boxes.iter().enumerate() {
        if b.len() > 0 {
            println!("{i}: {b:?}");
        }
    }
}

fn part2(input: &str) -> usize {
    let mut boxes: Vec<Vec<(&str, usize)>> = Vec::new();
    (0..255).for_each(|_|  boxes.push(Vec::new()) );

    parse(input).into_iter().for_each(|s| {
        let (label, instr, f) = split_instr(s);
        let i_box = hash(label.as_bytes());

        let b = &mut boxes[i_box];

        match (instr, b.iter().position(|(lbl, _)| *lbl == label)) {
            ('=', None)     => {b.push((label, f.unwrap())); },
            ('=', Some(i))  => { b[i] = (label, f.unwrap()); },
            ('-', None)     => (),
            ('-', Some(i))  => {b.remove(i); },
            _               => unreachable!(),
        }
        //println!("step: {s}");
        //print_boxes(&boxes);
    });

    boxes.iter().enumerate().map(|(i_box, b)| {
        b.iter().enumerate().map(move |(i_slot, (_, f))| {
            (i_box + 1) * (i_slot + 1) * f
        })
    }).flatten().sum()
}

#[cfg(test)]
mod tests {
    use super::*;


    #[test]
    fn hash_test() {
        assert_eq!(hash("HASH".as_bytes()), 52);
        assert_eq!(hash("rn".as_bytes()), 0);
    }

    #[test]
    fn hash_special_characters_test() {
        assert_eq!(hash("cm=2".as_bytes()), 47);
        assert_eq!(hash("ot=7".as_bytes()), 231);
    }

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 1320);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 511498);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 145);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 284674);
    }
}
