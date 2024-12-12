
use utils::{run_task, direction::Direction, map::{Map, SparseMap, Vec2}, took};


fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input));
    run_task!(|| part2(input));
}

fn part1(input: &str) -> usize {
    let map = SparseMap::from_str(input, &|c| Some(c));

    map.find_all(&|&v| v == 'X')
      .flat_map(|p| {
        Direction::DIRECTION_8.into_iter().map(|d| {
                (0..4).map(|i| map.get(&(p.advance_n(&d, i))).unwrap_or(&'-')).collect()
            }).collect::<Vec<String>>()
        })
     .filter(|v| v == "XMAS")
     .count()

}

fn is_x_mas(p: &Vec2, map: &SparseMap<char>) -> bool {

    let tl = map.get(&(*p + Vec2::new(-1, 1))).unwrap_or(&'-');
    let tr = map.get(&(*p + Vec2::new(1, 1))).unwrap_or(&'-');
    let bl = map.get(&(*p + Vec2::new(-1, -1))).unwrap_or(&'-');
    let br = map.get(&(*p + Vec2::new(1, -1))).unwrap_or(&'-');

    ((*tl, *br) == ('M', 'S') || (*tl, *br) == ('S', 'M')) && ((*tr, *bl) == ('M', 'S') || (*tr, *bl) == ('S', 'M'))
}

fn part2(input: &str) -> usize {
    let map = SparseMap::from_str(input, &|c| Some(c));

    // find all X
    let xs: Vec<_> = map.iter()
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
