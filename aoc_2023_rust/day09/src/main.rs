use took::took;
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

fn parse(input: &str) -> impl Iterator<Item = Vec<isize>> + '_ {
    input.lines().map(|l| {
        l.split_whitespace().map(|v| v.parse().unwrap()).collect()
    })

}

//fn predict<I>(it: I) -> isize
//    where I: IntoIterator<Item = isize> + Clone
//{
//
//    let last = it.clone().into_iter().next().unwrap();
//    let difs = it.clone().into_iter().map(|(a, b)| a - b);
//    if let Ok(v) = difs.clone().take(2).all_equal_value() {
//
//    }
//
//
//
//    line.windows(2).map(|w| w[1] - w[0]);
//
//    1
//}
//

fn predict(line: &Vec<isize>) -> isize {
    let diff: Vec<isize> = line.windows(2).map(|v| v[0] - v[1]).collect();
    if diff.iter().all_equal() {
        line[0] + diff[0]
    } else {
        line[0] + predict(&diff)
    }
}

fn part1(input: &str) -> isize {
    parse(input).map(|l| predict(&l.into_iter().rev().collect::<Vec<_>>())).sum()
}

fn part2(input: &str) -> isize {
    parse(input).map(|l| predict(&l.into_iter().collect::<Vec<_>>())).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 114);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 2);
    }
}
