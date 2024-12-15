use std::collections::{HashSet, VecDeque};

use utils::{direction::Direction, map::{Map, SparseMap, Vec2}, run_task, took};

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input));
    run_task!(|| part2(input));
}

fn parse(input: &str) -> (SparseMap<char>, Vec<Direction>) {
    let (map, directions) = input.split_once("\n\n").unwrap();

    (
        SparseMap::from_str(map, &|c| (c != '.').then_some(c)),
        directions.chars().filter_map(|c| Direction::try_from(&c).ok()).collect()
    )
}


fn part1(input: &str) -> usize {
    let (mut map, directions) = parse(input);
    let mut robot = map.find_all(|c| c == '@').next().unwrap();

    for d in directions {
        // New robot position
        let mut new_robot = Some(robot.advance(&d));
        // In case of boxes move, this is the new position of the box
        let mut new_box = None;
        loop {

            if let Some(b) = new_box {
                match map.get(&b) {
                    None => break,
                    Some('O') => {
                        new_box = Some(b.advance(&d))
                    }
                    Some('#') => {
                        new_robot = None;
                        new_box = None;
                        break;
                    },
                    v => unreachable!("this should not be: {v:?}"),
                }
            } else if let Some(r) = new_robot {
                match map.get(&r) {
                    None => break,
                    Some('O') => {
                        new_box = Some(r.advance(&d))
                    }
                    Some('#') => {
                        new_robot = None;
                        new_box = None;
                        break;
                    },
                    v => unreachable!("this should not be: {v:?}"),
                }
            }
        }

        if let Some(r) = new_robot {
            map.remove(&robot);
            map.insert(r, '@');
            robot = r;
        }

        if let Some(b) = new_box {
            map.insert(b, 'O');
        }
    }

    map.find_all(|c| c == 'O').map(|v| v.y * 100 + v.x).sum::<isize>() as usize
}

fn enlarge(map: &SparseMap<char>) -> SparseMap<char> {
    let mut new_map = SparseMap::new();

    let bounds = map.bounds();
    for y in bounds.min.y..=bounds.max.y  {
        for x in bounds.min.x..=bounds.max.x {
            match map.get(&(x, y).into()) {
                Some('#') => {
                    new_map.insert((2 * x, y).into(), '#');
                    new_map.insert((2 * x + 1, y).into(), '#');
                },
                Some('O') => {
                    new_map.insert((2 * x, y).into(), '[');
                    new_map.insert((2 * x + 1, y).into(), ']');
                }
                Some('@') => {
                    new_map.insert((2 * x, y).into(), '@');
                }
                _ => (),
            }
        }
    }
    new_map
}

fn get_boxes(pos: Vec2, d: Direction, map: &SparseMap<char>) -> Option<Vec<Vec2>> {
    let mut boxes = HashSet::new();
    let mut queue = VecDeque::new();

    queue.push_back(pos);
    while let Some(n) = queue.pop_front() {
        if !boxes.contains(&n) {
            match map.get(&n) {
                None => (),
                Some('#') => return None,
                Some('[') => {
                    boxes.insert(n);
                    boxes.insert(n.advance(&Direction::East));
                    queue.push_back(n.advance(&d));
                    queue.push_back(n.advance(&Direction::East).advance(&d));
                }
                Some(']') => {
                    boxes.insert(n);
                    boxes.insert(n.advance(&Direction::West));
                    queue.push_back(n.advance(&d));
                    queue.push_back(n.advance(&Direction::West).advance(&d));
                }
                v => unreachable!("this should not happeded: {v:?}")
            }
        }
    }

    Some(boxes.into_iter().collect())
}

fn part2(input: &str) -> usize {
    let (map, directions) = parse(input);
    let mut map = enlarge(&map);
    let mut robot = map.find_all(|c| c == '@').next().unwrap();

    for d in directions {

        // New robot position
        let new_robot = robot.advance(&d);
        if let Some(boxes) = get_boxes(new_robot, d, &map) {

            let boxes: Vec<_> = boxes.into_iter().map(|p| (p, *map.get(&p).unwrap())).collect();

            for (k, _) in boxes.iter() {
                map.remove(k);
            }

            for (k, c) in boxes {
                map.insert(k.advance(&d), c);
            }

            map.remove(&robot);
            map.insert(new_robot, '@');
            robot = new_robot;

        }
    }

    map.print('.');
    map.find_all(|c| c == '[').map(|v| v.y * 100 + v.x).sum::<isize>() as usize
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day15_part1_small_test() {
        let input = include_str!("../input_test_small.txt");
        assert_eq!(part1(input), 2028);
    }

    #[test]
    fn day15_part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 10092);
    }

    #[test]
    fn day15_part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 1463715);
    }

    #[test]
    fn day15_part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 9021);
    }

    #[test]
    fn day15_part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1481392);
    }
}
