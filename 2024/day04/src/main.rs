use std::{char, collections::HashMap};

use common::{aoc_task, map::{Direction, Map, Pos}, took};



fn main() {
    let input = include_str!("../input.txt");
    aoc_task!(|| part1(input));
    aoc_task!(|| part2(input));
}

fn parse(input: &str) -> HashMap<Pos, char> {
    let mut map: HashMap<Pos, char> = HashMap::new();
    input
        .lines()
        .enumerate()
        .for_each(|(y, l)| {
            l.chars()
             .enumerate()
             .for_each(|(x, c)| {
                    map.insert(Pos::new(x as i32, y as i32), c);
                })
        });
    map
}


fn part1(input: &str) -> usize {
    let map = parse(input);

    // find all X
    let xs: Vec<Pos> = map.iter()
        .filter_map(|(&k, &v)| {
            if v == 'X' {
                Some(k)
            } else {
                None
            }})
        .collect();

    // from every X go to all directions
    xs.into_iter()
      .flat_map(|p| {
        Direction::all().into_iter().map(|d| {
                (0..4).map(|i| map.get(&(p + Pos::from(&d).scale(i))).unwrap_or(&'-')).collect()
            }).collect::<Vec<String>>()
        })
     .filter(|v| v == "XMAS")
     .count()

}

fn part2(input: &str) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 18);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 1);
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
