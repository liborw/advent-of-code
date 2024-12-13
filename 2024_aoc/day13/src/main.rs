use std::collections::VecDeque;

use microlp::{ComparisonOp, OptimizationDirection, Problem};
use regex::Regex;
use utils::{run_task, took, vector::Vec2};

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input));
    run_task!(|| part2(input));
}

#[derive(Debug)]
struct Machine {
    button_a: Vec2<usize>,
    button_b: Vec2<usize>,
    prize: Vec2<usize>
}



impl Machine {

    // too slow
    fn play(&self) -> Option<usize> {
        let mut min_cost = None;
        let mut queue = VecDeque::new();
        queue.push_back((Vec2::zero(), 0));


        while let Some((v, c)) = queue.pop_front() {

            if c > min_cost.unwrap_or(usize::MAX) || v.x > self.prize.x || v.y > self.prize.y {
                continue;
            }

            if v == self.prize {
                min_cost = Some(c);
                continue;
            }

            queue.push_back((v + self.button_a, c + 3));
            queue.push_back((v + self.button_b, c + 1));
        };
        min_cost
    }

    fn solve(&self) -> Option<usize> {
        let nb = (self.prize.x / self.button_b.x).min(self.prize.y / self.button_b.y);

        for i in 0..nb {
            let rem = self.prize - self.button_b * (nb - i);
            if rem.is_zero() {
                println!("found: a: {0} b:{} c: {}", nb - i, nb - i);
                return Some(nb - i);
            }

            let na = (rem.x / self.button_a.x).min(rem.y / self.button_a.y);
            let rem = rem - self.button_a * na;


            if rem.is_zero() {
                println!("found: a: {na} b:{} c: {}", nb - i, nb - i + na * 3);
                return Some(nb - i + na * 3);
            }
        }
        None
    }

    fn solve_lp(&self) -> Option<usize> {
        let mut problem = Problem::new(OptimizationDirection::Minimize);
        let a = problem.add_integer_var(3.0, (0, i32::MAX));
        let b = problem.add_integer_var(1.0, (0, i32::MAX));

        problem.add_constraint([(a, self.button_a.x as f64), (b, self.button_b.x as f64)], ComparisonOp::Eq, self.prize.x as f64);
        problem.add_constraint([(a, self.button_a.y as f64), (b, self.button_b.y as f64)], ComparisonOp::Eq, self.prize.y as f64);

        problem.solve().map(|s| {
            let a_val = s.var_value_rounded(a) as usize;
            let b_val = s.var_value_rounded(b) as usize;
            let cost = a_val * 3 + b_val;
            println!("{self:?} a: {} ({a_val}), b: {} ({b_val}), obj: {} ({cost})", s.var_value(a), s.var_value(b), s.objective() );
            cost
        }).ok()
    }
}


fn parse(input: &str) -> Vec<Machine> {
    let re = Regex::new(r"Button A: X\+(\d*), Y\+(\d*)\nButton B: X\+(\d*), Y\+(\d*)\nPrize: X=(\d+). Y=(\d+)").unwrap();

    re.captures_iter(input)
        .map(|c| c.extract::<6>().1.into_iter()
            .map(|v| v.parse::<usize>().unwrap())
            .collect::<Vec<_>>())
        .map(|v| {
        Machine{
                button_a: Vec2::new(v[0], v[1]),
                button_b: Vec2::new(v[2], v[3]),
                prize: Vec2::new(v[4], v[5]),
            }
        }).collect()
}

// 1h
fn part1(input: &str) -> usize {
    let machines = parse(input);
    machines.into_iter().filter_map(|m| m.solve_lp()).sum()
}

fn part2(input: &str) -> usize {
    let mut machines = parse(input);
    for m in machines.iter_mut() {
        m.prize.x += 10000000000000;
        m.prize.y += 10000000000000;
    }
    machines.into_iter().filter_map(|m| m.solve_lp()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day13_part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 480);
    }

    #[test]
    fn day13_part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 29438);
    }

    #[test]
    fn day13_part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn day13_part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1);
    }
}
