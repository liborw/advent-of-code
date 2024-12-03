use common::{took, aoc_task};
use regex::Regex;



fn main() {
    let input = include_str!("../input.txt");
    aoc_task!(|| part1(input));
    aoc_task!(|| part2(input));
}

fn part1(input: &str) -> i32 {
    let mul_re = Regex::new(r"mul\((?<left>\d+),(?<right>\d+)\)").unwrap();
    mul_re
        .captures_iter(input)
        .map(|cap| {
            let left: i32 = cap["left"].parse().unwrap();
            let right: i32 = cap["right"].parse().unwrap();
            left * right
        }).sum()
}

fn part2(input: &str) -> i32 {
    let mul_re = Regex::new(r"mul\((?<left>\d+),(?<right>\d+)\)|do\(\)|don't\(\)").unwrap();
    let mut enable = true;

    mul_re
        .captures_iter(input)
        .map(|cap| {

            match &cap[0] {
                "do()" => {
                    enable = true;
                    0
                }
                "don't()" => {
                    enable = false;
                    0
                }
                _ if enable => {
                    let left: i32 = cap["left"].parse().unwrap();
                    let right: i32 = cap["right"].parse().unwrap();
                    left * right
                }
                _ => 0
            }
        }).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 161);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 184511516);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test2.txt");
        assert_eq!(part2(input), 48);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1);
    }
}
