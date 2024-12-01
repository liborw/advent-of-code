use itertools::Itertools;
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

fn parse_block(block: &str) -> Vec<(usize, usize, usize)> {
    block.lines().skip(1).map(|l| {
        l.split_whitespace().map(|v| v.parse::<usize>().unwrap()).collect_tuple().unwrap()
    }).collect()
}

fn part1(input: &str) -> usize {
    let mut blocks = input.split("\n\n");
    let mut seeds: Vec<usize> = blocks.next().unwrap()
                                         .split_once(": ").unwrap().1
                                         .split_whitespace().map(|v| v.parse().unwrap()).collect();

    blocks.for_each(|block| {
        let ranges: Vec<(usize, usize, usize)> = parse_block(block);
        seeds = seeds.iter().map(|s| {
            ranges.iter().find_map(|(a,b,c)| {
                s.checked_sub(*b).and_then(|v| {
                    if v < *c {
                        Some(a + v)
                    } else {
                        None
                    }
                })
            }).unwrap_or(*s)
        }).collect();
    });

    seeds.into_iter().min().unwrap()
}

fn part2(input: &str) -> usize {
    let mut blocks = input.split("\n\n");
    let mut seeds: Vec<(usize, usize)> = blocks.next().unwrap()
                                         .split_once(": ").unwrap().1
                                         .split_whitespace().map(|v| v.parse().unwrap())
                                         .tuples().map(|(a, b)| (a, a + b)).collect();

    blocks.for_each(|block| {
        let ranges: Vec<(usize, usize, usize)> = parse_block(block);
        let mut new: Vec<(usize, usize)> = Vec::new();

        while !seeds.is_empty() {
            let (s, e) = seeds.pop().unwrap();
            let mut found = false;
            for (ds, ss, rl) in ranges.iter() {
                let os = s.max(*ss);
                let oe = e.min(ss + rl);

                if os < oe {
                    new.push((os - ss + ds, oe - ss + ds));
                    if os > s { seeds.push((s, os)) }
                    if e > oe { seeds.push((oe, e)) }
                    found = true;
                    break;
                }
            }
            if !found { new.push((s, e)) }
        }
        seeds = new.clone();
    });

    seeds.into_iter().map(|(s, _)| s).min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 35);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 46);
    }
}
