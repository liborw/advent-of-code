use std::collections::HashSet;

use common::{map::{SparseMap, Map}, pos::Pos};
use took::took;

macro_rules! aoc_task {
    ($f:expr) => {
        let (took, result) = took($f);
        println!("{} took: {} result: {}", stringify!($f), took, result);
    };
}

fn main() {
    let input = include_str!("../input.txt");
    aoc_task!(|| part1(input));
    aoc_task!(|| part2(input));
}

fn parse(input: &str) -> SparseMap<char> {
    input.lines().enumerate().map(move |(x, l)| {
        l.chars().enumerate().filter_map(move |(y, c)| {
            Some(((x,y).into(), c))
        })
    }).flatten().collect()
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right
}


fn next_pos(pos: &Pos, dir: &Direction) -> Pos {

    match dir {
        Direction::Up       => *pos + (-1, 0).into(),
        Direction::Down     => *pos + ( 1, 0).into(),
        Direction::Left     => *pos + ( 0,-1).into(),
        Direction::Right    => *pos + ( 0, 1).into(),
    }
}

fn step(
    map: &SparseMap<char>,
    hit: &mut HashSet<(Pos, Direction)>,
    pos: Pos,
    dir: Direction
) -> () {

    if !map.contains_key(&pos) {
        return
    }

    if hit.contains(&(pos, dir)) {
        return
    } else {
        hit.insert((pos, dir));
    }

    match (map.get(&pos).unwrap(), dir) {
        ('.', d) => step(map, hit, next_pos(&pos, &d), d),

        ('\\', Direction::Left) => step(map, hit, next_pos(&pos, &Direction::Up), Direction::Up),
        ('\\', Direction::Right) => step(map, hit, next_pos(&pos, &Direction::Down), Direction::Down),
        ('\\', Direction::Up) => step(map, hit, next_pos(&pos, &Direction::Left), Direction::Left),
        ('\\', Direction::Down) => step(map, hit, next_pos(&pos, &Direction::Right), Direction::Right),

        ('/', Direction::Left) => step(map, hit, next_pos(&pos, &Direction::Down), Direction::Down),
        ('/', Direction::Right) => step(map, hit, next_pos(&pos, &Direction::Up), Direction::Up),
        ('/', Direction::Up) => step(map, hit, next_pos(&pos, &Direction::Right), Direction::Right),
        ('/', Direction::Down) => step(map, hit, next_pos(&pos, &Direction::Left), Direction::Left),

        ('|', d) if d == Direction::Up || d == Direction::Down => {
            step(map, hit, next_pos(&pos, &d), d);
        },

        ('|', d) if d == Direction::Left || d == Direction::Right => {
            step(map, hit, next_pos(&pos, &Direction::Down), Direction::Down);
            step(map, hit, next_pos(&pos, &Direction::Up), Direction::Up);
        },

        ('-', d) if d == Direction::Left || d == Direction::Right => {
            step(map, hit, next_pos(&pos, &d), d);
        },

        ('-', d) if d == Direction::Up || d == Direction::Down => {
            step(map, hit, next_pos(&pos, &Direction::Left), Direction::Left);
            step(map, hit, next_pos(&pos, &Direction::Right), Direction::Right);
        },
        _ => unreachable!()
    };
}


fn part1(input: &str) -> usize {
    let lenses = parse(input);
    let mut hit: HashSet<(Pos, Direction)> = HashSet::new();
    let dir = Direction::Right;
    let pos: Pos = (0,0).into();
    step(&lenses, &mut hit, pos, dir);
    let map: SparseMap<char> = hit.into_iter().map(|(p, _)| (p, '#')).collect();
    map.len()
}

fn part2(input: &str) -> usize {
    let lenses = parse(input);
    let mut hit: HashSet<(Pos, Direction)> = HashSet::new();
    let mut starts: Vec<(Pos, Direction)> = Vec::new();
    let bb = lenses.bounding_box();

    for x in 0..bb.x_max {
        starts.push(((x, bb.y_min).into(), Direction::Right));
    }

    for x in 0..=bb.x_max {
        starts.push(((x, bb.y_max).into(), Direction::Left));
    }

    for y in 0..=bb.y_max {
        starts.push(((bb.x_min, y).into(), Direction::Down));
    }

    for y in 0..=bb.y_max {
        starts.push(((bb.x_max, y).into(), Direction::Up));
    }

    starts.into_iter().map(|(pos, dir)| {
        hit.clear();
        step(&lenses, &mut hit, pos, dir);
        hit.iter().map(|&(p, _)| p).collect::<HashSet<Pos>>().len()
    }).max().unwrap()


}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 46);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 7482);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 51);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 7896);
    }
}
