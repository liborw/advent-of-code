use std::collections::HashSet;

use utils::{aoc_task, direction::{cardinal::Direction, AdvanceInDirection}, map::{Map, SparseMap, Vec2}, took};

fn main() {
    let input = include_str!("../input.txt");
    aoc_task!(|| part1(input));
    aoc_task!(|| part2(input));
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Guard {
    pos: Vec2,
    dir: Direction
}

impl Guard {
    fn new(pos: Vec2, dir: Direction) -> Self {
        Guard{pos, dir}
    }

    fn turn(&mut self) {
        self.dir = self.dir.turn_right();
    }

    fn r#move(&mut self) {
        self.pos = self.next_pos();
    }

    fn next_pos(&self) -> Vec2 {
        self.pos.advance(&self.dir)
    }
}


fn walk(guard: &Guard, map: &SparseMap<char>) -> (HashSet<Guard>, bool) {
    let mut g = guard.clone();
    let mut visited = HashSet::new();

    while map.get(&g.pos).is_some() {
        if map.get(&g.next_pos()) == Some(&'#') {

            if visited.contains(&g) {
                return (visited, true)
            }

            visited.insert(g.clone());

            // turn if needed
            while map.get(&g.next_pos()) == Some(&'#') {
                g.turn()
            }
        }
        visited.insert(g.clone());

        g.r#move();
    }
    (visited, false)
}

fn get_guard(map: &SparseMap<char>) -> Guard {
    let pos = map.find_all(&|c| "<>^v".chars().any(|v| v == *c)).next().unwrap();
    let dir = map.get(&pos).unwrap().try_into().unwrap();
    Guard::new(pos, dir)
}


fn part1(input: &str) -> usize {
    let map = SparseMap::from_str(input, &|v| Some(v));
    let guard = get_guard(&map);
    walk(&guard, &map).0.into_iter()
        .map(|g| g.pos)
        .collect::<HashSet<_>>()
        .len()
}

fn part2(input: &str) -> usize {
    let mut map = SparseMap::from_str(input, &|v| Some(v));
    let guard = get_guard(&map);

    walk(&guard, &map).0.into_iter()
        .map(|g| g.pos)
        .collect::<HashSet<_>>()
        .into_iter().filter(|p| {
            if *p == guard.pos {
                false
            } else {
                map.insert(*p, '#');
                let is_loop = walk(&guard, &map).1;
                map.insert(*p, '.');
                is_loop
            }
        }).count()

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 41);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 5453);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 6);
    }

    #[test]
    fn part2_test2() {
        let input = include_str!("../input_test2.txt");
        assert_eq!(part2(input), 5);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 2188);
    }
}
