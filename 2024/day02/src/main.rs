use common::{aoc_task, took};



fn main() {
    let input = include_str!("../input.txt");
    aoc_task!(|| part1(input));
    aoc_task!(|| part2(input));
}

fn parse(input: &str) -> Vec<Vec<i32>> {
    input.lines()
         .map(|line| {
            line.split_whitespace()
                .map(|v| v.parse::<i32>().unwrap())
                .collect()
        }).collect()
}


fn check(r: &[i32]) -> bool {
    let diff: Vec<_> = r.windows(2).map(|w| w[0] - w[1]).collect();
    let inc = diff.iter().all(|&v| v > 0 ) || diff.iter().all(|&v| v < 0);
    let high = diff.iter().all(|&v| v.abs() <= 3 && v.abs() > 0);
    inc && high
}

fn part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(|r| check(&r))
        .filter(|&x| x)
        .count()
}

fn part2(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(|r| {
            if check(&r) {
                true
            } else {
                (0..r.len()).any(|i| {
                    let mut r = r.clone();
                    r.remove(i);
                    check(&r)
                })
            }
        }).filter(|&x| x)
          .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 2);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 314);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 4);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1);
    }
}
