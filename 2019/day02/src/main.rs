use common::{took, aoc_task};



fn main() {
    let input = include_str!("../input.txt");
    aoc_task!(|| part1(input));
    aoc_task!(|| part2(input));
}

fn parse(input: &str) -> Vec<usize> {
    input.strip_suffix("\n").unwrap()
        .split(",")
        .map(|v| v.parse().unwrap())
        .collect()
}

fn part1(input: &str) -> usize {
    let mut program = parse(input);
    let mut cur = 0;

    program[1] = 12;
    program[2] = 2;

    while program[cur] != 99 {
        let a = program[cur + 1];
        let b = program[cur + 2];
        let i = program[cur + 3];

        match program[cur] {
            1 => program[i] = program[a] + program[b],
            2 => program[i] = program[a] * program[b],
            _ => unreachable!()
        }
        cur += 4;
    }
    program[0]
}

fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    //#[test]
    //fn part1_test() {
    //    let input = include_str!("../input_test.txt");
    //    assert_eq!(part1(input), 3500);
    //}

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 4138658);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1);
    }
}
