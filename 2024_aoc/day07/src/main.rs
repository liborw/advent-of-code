use utils::{took, run_task};

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input));
    run_task!(|| part2(input));
}

enum Op {
    Add,
    Mul,
    Con
}

impl Op {

    const P1: [Op; 2] = [Op::Add, Op::Mul];
    const P2: [Op; 3] = [Op::Add, Op::Mul, Op::Con];

    fn apply(&self, lhs: usize, rhs: usize) -> usize {
        match *self {
            Op::Add => lhs + rhs,
            Op::Mul => lhs * rhs,
            Op::Con => format!("{}{}", lhs, rhs).parse().unwrap()
        }
    }
}

type Equation = (usize, Vec<usize>);

fn parse(input: &str) -> Vec<Equation> {
    input.lines()
         .map(|l| {
            let (test, eq_str) = l.split_once(":").unwrap();
            let eq = eq_str.split_whitespace().map(|v| v.parse().unwrap()).collect();
            (test.parse().unwrap(), eq)
        }).collect()
}

fn test(cur: usize, ops: &[Op], rest: &[usize], check: usize) -> bool {

    if rest.is_empty() {
        return cur == check
    }

    if cur > check {
        return false
    }

    ops.iter().any(|op| {
        let next = op.apply(cur, rest[0]);
        test(next, ops, &rest[1..], check)
    })
}


fn part1(input: &str) -> usize {
    parse(input).into_iter().filter(|eq| {
        test(eq.1[0], &Op::P1, &eq.1[1..], eq.0)
    }).map(|v| v.0).sum()
}

fn part2(input: &str) -> usize {
    parse(input).into_iter().filter(|eq| {
        test(eq.1[0], &Op::P2, &eq.1[1..], eq.0)
    }).map(|v| v.0).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 3749);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 5837374519342);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 11387);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 492383931650959);
    }

}
