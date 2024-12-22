use std::{collections::{HashMap, HashSet}, mem::take, ops::{BitXor, Div}, usize};

use itertools::Itertools;
use utils::{took, run_task};

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input));
    run_task!(|| part2(input));
}

struct FunctionIterator<F, T>
where
    F: FnMut(T) -> T,
{
    current: T,
    func: F,
}

impl<F, T> FunctionIterator<F, T>
where
    F: FnMut(T) -> T,
{
    fn new(initial: T, func: F) -> Self {
        Self { current: initial, func }
    }
}

impl<F, T> Iterator for FunctionIterator<F, T>
where
    F: FnMut(T) -> T,
    T: Copy,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let result = self.current;
        self.current = (self.func)(self.current);
        Some(result)
    }
}

fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn mix(a: usize, b:usize) -> usize {
    b.bitxor(a)
}

fn prune(a: usize) -> usize {
    a % 16777216
}

fn step(init: usize) -> usize {

    // mul 64, mix and prune
    let a = prune(mix(init, init * 64));

    // div 32, round, mix and prune
    let b = prune(mix(a, a.div(32)));

    // mul 2048, mix and prune
    prune(mix(b, b * 2048))

}

fn price(num: usize) -> usize {
    num % 10
}

fn run(init: usize, n: usize) -> usize {
    (0..n).fold(init, |v, _| step(v))
}


fn part1(input: &str) -> usize {
    parse(input)
        .into_iter()
        .map(|n| run(n, 2000))
        .sum()
}

fn diff(vec: &[usize]) -> Vec<isize> {
    vec
        .windows(2)
        .map(|v| v[1] as isize - v[0] as isize)
        .collect()
}

fn gen(n: usize) -> impl Iterator<Item=usize> {
    // TODO: Include n in the secret
    let mut val = n;
    (0..).map(move |i| {
        if i != 0 {
            val = step(val);
        }
        val
    })
}


fn part2(input: &str) -> usize {
    let monkey: Vec<_> = parse(input)
        .into_iter()
        .map(|n| {
            let prices = gen(n)
                .take(2001)
                .map(price)
                .collect::<Vec<_>>();

            let mut monkey = HashMap::new();
            prices
                .windows(5)
                .for_each(|w| {
                    monkey.entry(diff(w)).or_insert(w[4]);
                });
            monkey
        }).collect();

    let unique: HashSet<_> = monkey
        .iter()
        .flat_map(|hm| hm.keys())
        .collect();

    unique
        .into_iter()
        .map(|k| {
            monkey
                .iter()
                .map(|hm| hm.get(k).unwrap_or(&0))
                .sum()
        }).max().unwrap()
}

#[cfg(test)]
mod tests {
    use itertools::Itertools;

    use super::*;

    #[test]
    fn day22_mix_test() {
        assert_eq!(mix(42, 15), 37);
    }

    #[test]
    fn day22_prune_test() {
        assert_eq!(prune(100000000), 16113920);
    }

    #[test]
    fn day22_step_test() {
        let mut num = 123;
        num = step(num);
        assert_eq!(num, 15887950);

        num = step(num);
        assert_eq!(num, 16495136);

        num = step(num);
        assert_eq!(num, 527345);

        num = step(num);
        assert_eq!(num, 704524);
    }

    #[test]
    fn day22_price_test() {
        let mut num = 123;
        num = step(num);
        assert_eq!(price(num), 0);

        num = step(num);
        assert_eq!(price(num), 6);

        num = step(num);
        assert_eq!(price(num), 5);

        num = step(num);
        assert_eq!(price(num), 4);
    }

    #[test]
    fn day22_diff_test() {
        let vec: Vec<_> = gen(123)
            .map(price)
            .tuple_windows()
            .map(|(a, b)| b as isize - a as isize)
            .take(9)
            .collect();

        assert_eq!(vec, vec![-3, 6, -1, -1, 0, 2, -2, 0, -2]);
    }



    #[test]
    fn day22_part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 37327623);
    }

    #[test]
    fn day22_part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 16039090236);
    }

    #[test]
    fn day22_part2_test() {
        let input = include_str!("../input_test2.txt");
        assert_eq!(part2(input), 23);
    }

    #[test]
    fn day22_part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1808);
    }
}
