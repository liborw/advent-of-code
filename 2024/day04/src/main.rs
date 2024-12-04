use std::{char, collections::HashMap};

use utils::{aoc_task, map::{Pos, Direction}, took};


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
                (0..4).map(|i| map.get(&(p.r#move(&d, i + 1))).unwrap_or(&'-')).collect()
            }).collect::<Vec<String>>()
        })
     .filter(|v| v == "XMAS")
     .count()

}

fn is_x_mas(p: &Pos, map: &HashMap<Pos, char>) -> bool {

    let tl = map.get(&(*p + Pos::new(-1, 1))).unwrap_or(&'-');
    let tr = map.get(&(*p + Pos::new(1, 1))).unwrap_or(&'-');
    let bl = map.get(&(*p + Pos::new(-1, -1))).unwrap_or(&'-');
    let br = map.get(&(*p + Pos::new(1, -1))).unwrap_or(&'-');

    ((*tl, *br) == ('M', 'S') || (*tl, *br) == ('S', 'M')) && ((*tr, *bl) == ('M', 'S') || (*tr, *bl) == ('S', 'M'))
}

fn part2(input: &str) -> usize {
    let map = parse(input);

    // find all X
    let xs: Vec<Pos> = map.iter()
        .filter_map(|(&k, &v)| {
            if v == 'A' {
                Some(k)
            } else {
                None
            }})
        .collect();

    xs.into_iter()
      .filter(|p| is_x_mas(p, &map))
      .count()
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
        assert_eq!(part1(input), 2532);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 9);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1941);
    }
}
