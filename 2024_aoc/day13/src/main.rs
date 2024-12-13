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

    #[allow(dead_code)]
    fn solve(&self) -> Option<usize> {
        let nb = usize::min(self.prize.x / self.button_b.x, self.prize.y / self.button_b.y);

        for i in 0..nb {
            let rem = self.prize - self.button_b * (nb - i);
            if rem.is_zero() {
                println!("found: a: {0} b:{} c: {}", nb - i, nb - i);
                return Some(nb - i);
            }

            let na = rem.x / self.button_a.x;
            let rem = rem - self.button_a * na;

            if rem.is_zero() {
                println!("found: a: {na} b:{} c: {}", nb - i, nb - i + na * 3);
                return Some(nb - i + na * 3);
            }
        }
        None
    }

    fn solve_tictac(&self) -> Option<usize> {
        let mut rem = self.prize;
        let mut last_est: Option<(usize, usize)> = None;
        let mut a_est = 1;
        let mut b_est = 1;

        while !last_est.is_some_and(|(a, b)| a == a_est && b == b_est) {

            let r = self.prize - self.button_a * a_est - self.button_b * b_est;
            if r.x == 0 && r.y == 0 {
                return Some(a_est * 3 + b_est);
            }

            // This is a hack
            let r = self.prize - self.button_a * (a_est + 1) - self.button_b * (b_est - 1);
            if r.x == 0 && r.y == 0 {
                return Some((a_est + 1) * 3 + (b_est - 1));
            }

            last_est = Some((a_est, b_est));

            b_est = usize::min(rem.x / self.button_b.x, rem.y / self.button_b.y);
            rem = self.prize - self.button_b * b_est;
            a_est = usize::max(rem.x / self.button_a.x, rem.y / self.button_a.y);
            rem = self.prize - self.button_a * a_est;
            b_est = usize::min(rem.x / self.button_b.x, rem.y / self.button_b.y);

            println!("a_est: {} b_est: {} rem: {}", a_est, b_est, self.prize - self.button_a * a_est - self.button_b * b_est);
        }

        None
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
    machines.into_iter().filter_map(|m| m.solve_tictac()).sum()
}

fn part2(input: &str) -> usize {
    let mut machines = parse(input);
    for m in machines.iter_mut() {
        m.prize.x += 10000000000000;
        m.prize.y += 10000000000000;
    }
    machines.into_iter().filter_map(|m| m.solve_tictac()).sum()
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
       assert_eq!(part2(input), 875318608908);
   }

   #[test]
   fn day13_part2_final_test() {
       let input = include_str!("../input.txt");
       assert_eq!(part2(input), 104958599303720);

   }
}
