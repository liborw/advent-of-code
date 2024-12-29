use utils::{took, run_task};

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input));
}

fn parse(input: &str) -> (Vec<[usize; 5]>, Vec<[usize; 5]>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    input
        .split("\n\n")
        .for_each(|block| {
            let mut code = [0; 5];
            let mut lines = block.lines();
            let is_lock = lines.next().unwrap().starts_with("#");

            lines
                .take(5)
                .for_each(|l| {
                    l.chars()
                     .enumerate()
                     .for_each(|(i, ch)| {
                            if ch == '#' {
                                code[i] += 1;
                            }
                     });
                });

            if is_lock {
                locks.push(code);
            } else {
                keys.push(code);
            }
        });


    (locks, keys)
}

fn fits(lock: &[usize; 5], key:  &[usize; 5]) -> bool {
    (0..5).all(|i| lock[i] + key[i] <= 5)
}

fn part1(input: &str) -> usize {
    let (locks, keys) = parse(input);

    let mut n = 0;
    for l in locks.iter() {
     for k in keys.iter() {
            if fits(l, k) {
                n += 1;
            }
        }
    }

    n
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day25_part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 3);
    }

    #[test]
    fn day25_part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 3451);
    }
}
