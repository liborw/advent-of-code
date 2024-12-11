use rustc_hash::FxHashMap as HashMap;
use utils::{math::digits, run_task, took};

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input));
    run_task!(|| part2(input));
}

fn parse(input: &str) -> Vec<usize> {
    input.split_whitespace().map(|v| v.parse().unwrap()).collect()
}

fn undigits(dig: &[u8]) -> usize {
    let mut v = 0;
    for d in dig {
        v = v * 10 + (*d as usize);
    }
    v
}

fn split(v: usize) -> Option<(usize, usize)> {
    let d: Vec<_> = digits(v).collect();
    if d.len() % 2 != 0 {
        None
    } else {
        let (a, b) = d.split_at(d.len() / 2);
        Some((undigits(a), undigits(b)))
    }
}



fn blink(stone: usize, n: usize, cache: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(v) = cache.get(&(stone, n)) {
        return *v
    }

    if n == 0 {
        return 1;
    }

    let v = if stone == 0 {
        blink(1, n - 1, cache)
    } else if let Some((a, b)) = split(stone) {
        blink(a, n - 1, cache) + blink(b, n - 1, cache)
    } else {
        blink(stone * 2024, n - 1, cache)
    };

    cache.insert((stone, n), v);
    v
}

fn part1(input: &str) -> usize {
    let stones = parse(input);
    let mut cache = HashMap::default();
    stones.into_iter().map(|s| blink(s, 25, &mut cache)).sum()
}

fn part2(input: &str) -> usize {
    let stones = parse(input);
    let mut cache = HashMap::default();
    stones.into_iter().map(|s| blink(s, 75, &mut cache)).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day11_split_test() {
        assert_eq!(split(22), Some((2, 2)));
        assert_eq!(split(222), None);
        assert_eq!(split(1234), Some((12, 34)));
    }

    #[test]
    fn day11_blink_test() {
        assert_eq!(blink(0, 2, &mut HashMap::default()), 1);
        assert_eq!(blink(2222, 2, &mut HashMap::default()), 4);
    }

    #[test]
    fn day11_part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 55312);
    }

    #[test]
    fn day11_part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 218079);
    }

    #[test]
    fn day11_part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 65601038650482);
    }

    #[test]
    fn day11_part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 259755538429618);
    }
}
