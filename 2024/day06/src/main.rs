use utils::{aoc_task, map::Direction, map::{Map, Pos, SparseMap}, took};

fn main() {
    let input = include_str!("../input.txt");
    aoc_task!(|| part1(input));
    aoc_task!(|| part2(input));
}

type Guard = (Pos, Direction);

fn walk(guard: &Guard, map: &mut SparseMap<char>) {
    let mut pos = guard.0;
    let mut dir = guard.1;

    while map.get(&pos).is_some() {
        map.insert(pos, 'X');

        let mut new_pos = pos.r#move(&dir, 1);
        while map.get(&new_pos) == Some(&'#') {
            dir = dir.turn_right_90();
            new_pos = pos.r#move(&dir, 1);
        }
        pos = new_pos;
    }
}

fn get_guard(map: &SparseMap<char>) -> Guard {
    let pos = map.find_all(&|c| "<>^v".chars().any(|v| v == *c)).next().unwrap();
    let dir = map.get(&pos).unwrap().try_into().unwrap();
    (pos, dir)
}


fn part1(input: &str) -> usize {
    let mut map = SparseMap::from_str(input, &|v| Some(v));
    let guard = get_guard(&map);
    walk(&guard, &mut map);
    map.print(' ');
    map.find_all(&|c| *c == 'X').count()
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
        assert_eq!(part2(input), 1);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1);
    }
}
