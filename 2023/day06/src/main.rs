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

fn parse1(input: &str) -> Vec<(usize, usize)> {

    let data: Vec<Vec<usize>> = input.lines().map(|l| {
        l.split_once(":").unwrap().1.split_whitespace().map(|v| {
            v.parse::<usize>().unwrap()
        }).collect()
    }).collect();

    data[0].clone().into_iter().zip(data[1].clone().into_iter()).collect()
}

fn part1(input: &str) -> usize {
    parse1(input).into_iter().map(|(t, d)| {
        (0..t).filter_map(|p| {
            if (p * (t - p)) > d {
                Some(p)
            } else {
                None
            }
        }).count()

    }).reduce(|acc, v| acc * v).unwrap()
}

fn parse2(input: &str) -> (usize, usize) {
    input.lines().map(|l| {
        l.split_once(":").unwrap().1.replace(" ", "").parse::<usize>().unwrap()
    }).collect_tuple().unwrap()
}

fn part2(input: &str) -> usize {
    let (t, d) = parse2(input);
    (0..t).filter_map(|p| {
        if (p * (t - p)) > d {
            Some(p)
        } else {
            None
        }
    }).count()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(parse2(input), (71530, 940200));
    }

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 288);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 71503);
    }
}
