use std::{env, fs};
use utils::{took, run_task};


fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(args[1].clone()).unwrap();
    run_task!(|| solve(input.as_str()));
}

fn solve(input: &str) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn {{ project-name }}_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(solve(input), 0);
    }

    #[test]
    fn {{ project-name }}_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(solve(input), 0);
    }

}
