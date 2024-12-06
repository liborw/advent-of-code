use std::collections::{HashMap, HashSet};

use utils::{aoc_task, map::{Direction, Map, Pos, SparseMap}, took};

fn main() {
    let input = include_str!("../input.txt");
    aoc_task!(|| part1(input));
    aoc_task!(|| part2(input));
}

type Guard = (Pos, Direction);
type Path = HashSet<(Pos, Direction)>;
type Cycles = HashMap<(Pos, Direction), bool>;

fn walk(guard: &Guard, map: &SparseMap<char>) -> Path {
    let mut pos = guard.0;
    let mut dir = guard.1;
    let mut path = Path::new();

    while map.get(&pos).is_some() {
        path.insert((pos, dir));
        let mut new_pos = pos.r#move(&dir, 1);
        while map.get(&new_pos) == Some(&'#') {
            dir = dir.turn_right_90();
            new_pos = pos.r#move(&dir, 1);
        }
        pos = new_pos;
    }
    path
}



fn get_guard(map: &SparseMap<char>) -> Guard {
    let pos = map.find_all(&|c| "<>^v".chars().any(|v| v == *c)).next().unwrap();
    let dir = map.get(&pos).unwrap().try_into().unwrap();
    (pos, dir)
}


fn part1(input: &str) -> usize {
    let map = SparseMap::from_str(input, &|v| Some(v));
    let guard = get_guard(&map);
    walk(&guard, &map).into_iter().map(|v| v.0).collect::<HashSet<Pos>>().len()
}

fn walk_with_cycles(pos: Pos, dir: Direction, cycles: &Cycles, map: &SparseMap<char>) -> (Path, bool) {
    let mut pos = pos;
    let mut dir = dir;
    let mut path = Path::new();

    while map.get(&pos).is_some() {

        match (cycles.get(&(pos, dir)), path.contains(&(pos, dir))) {
            (Some(v), _) => return (path, *v),
            (None, true) => return (path, true),
            _ => ()
        }
        path.insert((pos, dir));

        let mut new_pos = pos.r#move(&dir, 1);
        while map.get(&new_pos) == Some(&'#') {
            dir = dir.turn_right_90();
            new_pos = pos.r#move(&dir, 1);
        }
        pos = new_pos;
    }
    (path, false)
}

fn find_obstacles(guard: &Guard, map: &mut SparseMap<char>) -> Vec<Pos> {
    let mut pos = guard.0;
    let mut dir = guard.1;
    let mut cycles = Cycles::new();
    let mut obstacles = HashSet::new();

    while map.get(&pos).is_some() {
        let mut new_pos = pos.r#move(&dir, 1);

        // try to add obstacle
        let dir_test = dir.turn_right_90();
        if map.get(&new_pos) == Some(&'.') {
            map.insert(new_pos, '#');
            let (path, is_cycle) = walk_with_cycles(pos, dir_test, &cycles, map);
            map.insert(new_pos, '.');

            if is_cycle {
                //path.into_iter().for_each(|k| {cycles.insert(k, is_cycle);});
                obstacles.insert(new_pos);
            }
        }

        cycles.insert((pos, dir), true);
        while map.get(&new_pos) == Some(&'#') {
            dir = dir.turn_right_90();
            new_pos = pos.r#move(&dir, 1);
        }
        pos = new_pos;
    }
    obstacles.into_iter().collect()
}

fn part2(input: &str) -> usize {
    let mut map = SparseMap::from_str(input, &|v| Some(v));
    let guard = get_guard(&map);
    let obstacles = find_obstacles(&guard, &mut map);
    map.print('.');
    println!();

    obstacles.iter().for_each(|p| {map.insert(*p, 'O');});
    map.print('.');
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
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1);
    }
}
