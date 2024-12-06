use std::{collections::{HashMap, HashSet}};

use utils::{aoc_task, map::{Direction, Map, Pos, SparseMap}, took};

fn main() {
    let input = include_str!("../input.txt");
    aoc_task!(|| part1(input));
    aoc_task!(|| part2(input));
}

#[derive(Clone, PartialEq, Eq, Hash)]
struct Guard {
    pos: Pos,
    dir: Direction
}

impl Guard {
    fn new(pos: Pos, dir: Direction) -> Self {
        Guard{pos, dir}
    }

    fn turn(&mut self) {
        self.dir = self.dir.turn_right_90();
    }

    fn r#move(&mut self) {
        self.pos = self.pos.r#move(&self.dir, 1);
    }

    fn next_pos(&self) -> Pos {
        self.pos.r#move(&self.dir, 1)
    }
}

type Path = HashSet<Guard>;


fn walk(guard: &Guard, map: &SparseMap<char>) -> Path {
    let mut g = guard.clone();
    let mut path = Path::new();

    while map.get(&g.pos).is_some() {
        path.insert(g.clone());

        // turn if needed
        while map.get(&g.next_pos()) == Some(&'#') {
            g.turn()
        }
        path.insert(g.clone());
        g.r#move();
    }
    path
}

fn get_guard(map: &SparseMap<char>) -> Guard {
    let pos = map.find_all(&|c| "<>^v".chars().any(|v| v == *c)).next().unwrap();
    let dir = map.get(&pos).unwrap().try_into().unwrap();
    Guard::new(pos, dir)
}


fn part1(input: &str) -> usize {
    let map = SparseMap::from_str(input, &|v| Some(v));
    let guard = get_guard(&map);
    walk(&guard, &map).into_iter().map(|g| g.pos).collect::<HashSet<Pos>>().len()
}

fn is_cycle(guard: &Guard, map: &SparseMap<char>) -> bool {
    let mut g = guard.clone();
    let mut visited = HashSet::new();

    while map.get(&g.pos).is_some() {

        if visited.contains(&g) {
            return true
        }
        visited.insert(g.clone());

        // turn if needed
        while map.get(&g.next_pos()) == Some(&'#') {
            g.turn()
        }
        visited.insert(g.clone());
        g.r#move();
    }
    false
}

fn part2(input: &str) -> usize {
    let map = SparseMap::from_str(input, &|v| Some(v));
    let guard = get_guard(&map);

    map.find_all(&|v| v == &'.').filter(|p| {
        let mut map = map.clone();
        map.insert(*p, '#');
        is_cycle(&guard, &map)
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
        assert_eq!(part2(input), 2118);
    }
}
