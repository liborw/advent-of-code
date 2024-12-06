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

fn walk_with_cycle(guard: &Guard, path: &Path, map: &SparseMap<char>) -> bool {
    let mut g = guard.clone();
    let mut path = path.clone();

    while map.get(&g.pos).is_some() {

        if path.contains(&g) {
            return true
        }
        path.insert(g.clone());

        // turn if needed
        while map.get(&g.next_pos()) == Some(&'#') {
            g.turn()
        }
        path.insert(g.clone());
        g.r#move();
    }
    false
}

fn find_obstacles(guard: &Guard, map: &SparseMap<char>) -> Vec<Pos> {
    let mut g = guard.clone();
    let mut path = Path::new();
    let mut obstacles = HashSet::new();
    let mut map = map.clone();

    while map.contains_key(&g.pos) {
        let test_pos = g.next_pos();

        // try to add obstacle
        if map.get(&test_pos) == Some(&'.') {
            map.insert(test_pos, '#');
            if walk_with_cycle(&g, &path, &map) {
                obstacles.insert(test_pos);
            }
            map.insert(test_pos, '.');
        }

        path.insert(g.clone());

        // turn if needed
        while map.get(&g.next_pos()) == Some(&'#') {
            g.turn()
        }
        path.insert(g.clone());
        g.r#move();
    }
    obstacles.into_iter().collect()
}

fn draw_cyrcles(guard: &Guard, map: &SparseMap<char>) -> SparseMap<char> {
    let mut g = guard.clone();
    let mut path = Path::new();
    let mut map = map.clone();

    while map.contains_key(&g.pos) {

        if path.contains(&g) {
            map.insert(g.pos, '+');
            break
        }

        path.insert(g.clone());

        // turn if needed
        let mut new_g = g.clone();
        while map.get(&new_g.next_pos()) == Some(&'#') {
            new_g.turn()
        }

        path.insert(new_g.clone());
        new_g.r#move();

        let label = if new_g.dir != g.dir || map.get(&g.pos) == Some(&'|') || map.get(&g.pos) == Some(&'-') {
            '+'
        } else {
            match g.dir {
                Direction::North | Direction::South => '|',
                Direction::West | Direction::East => '-',
                _ => ' '
            }
        };
        map.insert(g.pos, label);

        g = new_g;

    }
    map

}

fn part2(input: &str) -> usize {
    let map = SparseMap::from_str(input, &|v| Some(v));
    let guard = get_guard(&map);
    let obstacles = find_obstacles(&guard, &map);

    // debug

    // for o in obstacles.iter() {
    //     let mut map = map.clone();
    //     map.insert(*o, '#');
    //     map = draw_cyrcles(&guard, &map);
    //     map.insert(*o, 'O');
    //     map.insert(guard.pos, '^');
    //     map.print('X');
    //     println!();
    // }

    obstacles.len()
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
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1);
    }
}
