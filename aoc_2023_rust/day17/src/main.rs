use std::collections::{VecDeque, HashMap, BinaryHeap, HashSet};
use std::cmp::{Reverse, Ordering};
use std::fmt::Display;

use common::{map::{SparseMap, Map}, pos::Pos, direction::Direction};
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

fn parse(import: &str) -> SparseMap<usize> {
    import.lines().enumerate().map(|(x, l)| {
        l.chars().enumerate().map(move |(y, c)| {
            ((x, y).into(), c.to_digit(10).unwrap() as usize)
        })
    }).flatten().collect()
}


#[derive(Debug, Copy, Clone, Eq, PartialEq)]
struct State {
    pos: Pos,
    dir: Direction,
    step: usize,
    cost: usize
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl State {
    fn new(pos: Pos, dir: Direction, step: usize, cost: usize) -> Self {
        State{pos, dir, step, cost }
    }

    fn key(&self) -> (Pos, Direction, usize) {
        (self.pos, self.dir, self.step)
    }
}

impl Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?} {:?} {} {}", self.pos, self.dir, self.step, self.cost)
    }
}

fn part1(input: &str) -> usize {
    let map = parse(input);

    let mut dist: HashMap<Pos, usize> = HashMap::new();
    dist.insert((0,0).into(), 0);

    let mut open: BinaryHeap<State> = BinaryHeap::new();
    open.push(State::new((0, 0).into(), Direction::Right, 1, 0));
    open.push(State::new((0, 0).into(), Direction::Down, 1, 0));

    let mut closed: HashSet<(Pos, Direction, usize)> = HashSet::new();

    while !open.is_empty() {
        let state = open.pop().unwrap();


        if closed.contains(&state.key()) {
            continue;
        }

        println!("{state}");

        // Add current node to closed
        closed.insert(state.key());

        let next_pos = state.dir.move_pos(&state.pos);
        if let Some(next_pos_e) = map.get(&next_pos) {
            let next_e = state.cost + next_pos_e;


            let &cur_dist = dist.get(&next_pos).unwrap_or(&usize::MAX);
            if cur_dist > next_e {
                dist.insert(next_pos, next_e);
            }

            // Add ne nodes to open
            if state.step < 3 {
                open.push(State::new(next_pos, state.dir, state.step+1, next_e));
            }
            open.push(State::new(next_pos, state.dir.rotate_left(), 1, next_e));
            open.push(State::new(next_pos, state.dir.rotate_right(), 1, next_e));
        }
    };
    println!("{dist:?}");

    let bb = map.bounding_box();
    *dist.get(&(bb.x_max, bb.x_max).into()).unwrap()
}

fn part2(input: &str) -> usize {
    let map = parse(input);

    let mut dist: HashMap<Pos, usize> = HashMap::new();
    dist.insert((0,0).into(), 0);

    let mut open: BinaryHeap<State> = BinaryHeap::new();
    open.push(State::new((0, 0).into(), Direction::Right, 1, 0));
    open.push(State::new((0, 0).into(), Direction::Down, 1, 0));

    let mut closed: HashSet<(Pos, Direction, usize)> = HashSet::new();

    while !open.is_empty() {
        let state = open.pop().unwrap();


        if closed.contains(&state.key()) {
            continue;
        }

        println!("{state}");

        // Add current node to closed
        closed.insert(state.key());

        let next_pos = state.dir.move_pos(&state.pos);
        if let Some(next_pos_e) = map.get(&next_pos) {
            let next_e = state.cost + next_pos_e;


            let &cur_dist = dist.get(&next_pos).unwrap_or(&usize::MAX);
            if cur_dist > next_e {
                dist.insert(next_pos, next_e);
            }

            // Add ne nodes to open
            if state.step >= 4 {
                open.push(State::new(next_pos, state.dir.rotate_left(), 1, next_e));
                open.push(State::new(next_pos, state.dir.rotate_right(), 1, next_e));
            }

            if state.step < 10 {
                open.push(State::new(next_pos, state.dir, state.step+1, next_e));
            }
        }
    };
    println!("{dist:?}");

    let bb = map.bounding_box();
    *dist.get(&(bb.x_max, bb.x_max).into()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_min() {
        let input = include_str!("../input_test_min.txt");
        assert_eq!(part1(input), 6);
    }

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 102);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 1138);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 94);
    }

    #[test]
    fn part2_test_bad() {
        let input = include_str!("../input_test_bad.txt");
        assert_eq!(part2(input), 71);
    }

   //  #[test]
   //  fn part2_final_test() {
   //      let input = include_str!("../input.txt");
   //      assert_eq!(part2(input), 1);
   //  }
}
