use common::{map::SparseMap, map::Map, pos::Pos};
use took::took;

macro_rules! aoc_task {
    ($f:expr) => {
        let (took, result) = took($f);
        println!("{} took: {} result: {}", stringify!($f), took, result);
    };
}

fn main() {
    let input = include_str!("../input.txt");
    aoc_task!(|| part1(input, 1));
    aoc_task!(|| part1(input, 1000000-1));
}

fn parse(input: &str) -> SparseMap<char> {
    input.lines().enumerate().map(|(x, l)| {
        l.chars().enumerate().filter_map(move |(y, c)| {
            if c != '.' {
                Some(((x, y).into(), c))
            } else {
                None
            }
        })
    }).flatten().collect()
}

fn expansion(map: SparseMap<char>, n: usize) -> SparseMap<char> {
    let mut map = map;
    let mut bb = map.bounding_box();
    let mut x = bb.x_min;
    while x <= bb.x_max {
        if map.keys().all(|p| p.x != x) {
            bb.x_max += n as isize;
            x += n as isize;
            map = map.into_iter().map(|(p, c)| {
                if p.x > (x - n as isize) {
                    (p + (n, 0).into(), c)
                } else {
                    (p, c)
                }
            }).collect()
        }
        x += 1;
    }

    let mut y = bb.y_min;
    while y <= bb.y_max {
        if map.keys().all(|p| p.y != y) {
            bb.y_max += n as isize;
            y += n as isize;
            map = map.into_iter().map(|(p, c)| {
                if p.y > (y - n as isize) {
                    (p + (0, n).into(), c)
                } else {
                    (p, c)
                }
            }).collect()
        }
        y += 1;
    }
    map
}

fn part1(input: &str, n: usize) -> usize {
    let mut map = parse(input);
    map = expansion(map, n);
    let galaxies: Vec<Pos> = map.into_keys().collect();
    let mut total_dist = 0;
    for i in 0..galaxies.len() {
        for j in i..galaxies.len() {
            total_dist += galaxies[i].dist_manhatan(galaxies[j]);
        }
    }
    total_dist
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input, 1), 374);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input, 1), 9693756);
    }

    #[test]
    fn part2_test1() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input, 10-1), 1030);
    }

    #[test]
    fn part2_test2() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input, 100-1), 8410);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input, 1000000-1), 717878258016);
    }
}
