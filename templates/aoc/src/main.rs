use utils::{took, run_task};

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input));
    run_task!(|| part2(input));
}

fn part1(input: &str) -> usize {
    0
}

fn part2(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn {{ project-name }}_part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn {{ project-name }}_part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 1);
    }

    #[test]
    fn {{ project-name }}_part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn {{ project-name }}_part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1);
    }
}
