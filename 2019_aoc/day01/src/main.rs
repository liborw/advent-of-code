use common::{took, aoc_task};

fn main() {
    let input = include_str!("../input.txt");
    aoc_task!(|| part1(input));
    aoc_task!(|| part2(input));
}

fn how_much_fuel(v:i32) -> i32 {
    let fuel = (v / 3) - 2;

    if fuel <= 0 {
        0
    } else {
        fuel
    }
}

fn part1(input: &str) -> i32 {
    input
        .lines()
        .map(|v| v.parse::<i32>().unwrap())
        .map(how_much_fuel)
        .sum()
}

fn part2(input: &str) -> i32 {
    input
        .lines()
        .map(|v| v.parse::<i32>().unwrap())
        .map(|v| {
            let mut totalfuel = 0;
            let mut mass = v;

            while mass > 0 {
                mass = how_much_fuel(mass);
                totalfuel += mass
            }
            totalfuel
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 2 + 2 + 654 + 33583);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 3514064);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 51316);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 5268207);
    }
}
