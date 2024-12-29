use core::panic;
use std::{collections::HashMap, fmt::Debug, hash::Hash};

use itertools::Itertools;
use pathfinding::prelude::{astar, astar_bag, bfs};
use utils::{map::{Map, SparseMap, Vec2}, run_task, took};

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input));
    run_task!(|| part2(input));
}





#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Move {
    Up = 0,
    Down = 1,
    Left = 2,
    Right = 3,
    Press = 4
}

impl From<Move> for char {
    fn from(value: Move) -> Self {
        use Move::*;
        match value {
            Left => '<',
            Right => '>',
            Up => '^',
            Down => 'v',
            Press => 'A',
        }
    }
}

impl From<char> for Move {
    fn from(value: char) -> Self {
        use Move::*;
        match value {
            '^' => Up,
            'v' => Down,
            '<' => Left,
            '>' => Right,
            'A' => Press,
            _ => panic!("Not possible")
        }
    }
}

impl From<Move> for Vec2 {
    fn from(value: Move) -> Self {
        use Move::*;
        match value {
            Up => (0, -1).into(),
            Down => (0, 1).into(),
            Left => (-1, 0).into(),
            Right => (1, 0).into(),
            Press => (0, 0).into(),
        }
    }
}

impl From<Vec2> for Move {
    fn from(value: Vec2) -> Self {
        match (value.x, value.y) {
            (0, -1) => Move::Up,
            (0, 1) => Move::Down,
            (-1, 0) => Move::Left,
            (1, 0) => Move::Right,
            (0, 0) => Move::Press,
            _ => panic!("This shoulf not happend")
        }
    }
}

impl Move {
    const SHIFTS: [Move; 4] = [
        Move::Up,
        Move::Left,
        Move::Right,
        Move::Down,
    ];
}


#[derive(Clone)]
struct Keypad {
    map: SparseMap<char>,
}

#[derive(Debug, Clone, Eq, Hash)]
struct State {
    pos: Vec2,
    pressed: bool,
    prev: Move
}

impl State {
    fn new(pos: Vec2, pressed: bool, prev: Move) -> Self {
        State{pos, pressed, prev}
    }
}

impl PartialEq for State {
    fn eq(&self, other: &Self) -> bool {
        self.pos == other.pos && self.pressed == other.pressed
    }
}


impl Keypad {

    fn numeric() -> Self {
        let mut map = SparseMap::new();
        map.insert((0, 0).into(), '7');
        map.insert((1, 0).into(), '8');
        map.insert((2, 0).into(), '9');
        map.insert((0, 1).into(), '4');
        map.insert((1, 1).into(), '5');
        map.insert((2, 1).into(), '6');
        map.insert((0, 2).into(), '1');
        map.insert((1, 2).into(), '2');
        map.insert((2, 2).into(), '3');
        map.insert((1, 3).into(), '0');
        map.insert((2, 3).into(), 'A');
        Keypad{map}
    }

    fn directional() -> Self {
        let mut map = SparseMap::new();
        map.insert((1, 0).into(), '^');
        map.insert((2, 0).into(), 'A');
        map.insert((0, 1).into(), '<');
        map.insert((1, 1).into(), 'v');
        map.insert((2, 1).into(), '>');
        Keypad{map}
    }

    fn path_with_cost(&self, start: char, goal: char, costs: &Costs) -> (String, usize) {
        let start_pos = self.map.find_first(|c| c == start).unwrap();
        let goal_pos = self.map.find_first(|c| c == goal).unwrap();

        let (path, cost) = astar(
                &State::new(start_pos, false, Move::Press),
                |n| {
                    println!("{n:?}");
                    let mut out = vec![];
                    if n.pos == goal_pos {
                        let cost = costs.get(&(n.prev.into(), 'A')).unwrap().1;
                        out.push((State::new(n.pos, true, Move::Press), cost));
                    } else {
                        Move::SHIFTS.into_iter()
                            .for_each(|m| {
                                let test = n.pos.advance(m);
                                if self.map.contains_key(&test) {
                                    let cost = costs.get(&(n.prev.into(), m.into())).unwrap().1;
                                    println!(" {test} {m:?} {cost}");
                                    out.push((State::new(test, false, m), cost));
                                }
                            });
                    }
                    out
                },
                |n| (n.pos - goal_pos).manhatan() as usize,
                |n| n.pressed
            ).unwrap();
        (path.into_iter().map(|s| char::from(s.prev)).skip(1).collect(), cost)
    }

    fn path_with_cost2(&self, start: char, goal: char, costs: &Costs) -> (String, usize) {
        let start_pos = self.map.find_first(|c| c == start).unwrap();
        let goal_pos = self.map.find_first(|c| c == goal).unwrap();

        let (path, cost) = astar(
                &State::new(start_pos, false, Move::Press),
                |n| {
                    let mut out = vec![];
                    if n.pos == goal_pos {
                        let cost = costs.get(&(n.prev.into(), 'A')).unwrap().1;
                        out.push((State::new(n.pos, true, Move::Press), cost));
                    } else {
                        Move::SHIFTS.into_iter()
                            .for_each(|m| {
                                let test = n.pos.advance(m);
                                if self.map.contains_key(&test) {
                                    let cost = costs.get(&(n.prev.into(), m.into())).unwrap().1;
                                    out.push((State::new(test, false, m), cost));
                                }
                            });
                    }
                    out
                },
                |n| (n.pos - goal_pos).manhatan() as usize,
                |n| n.pressed
            ).unwrap();
        (path.into_iter().map(|s| char::from(s.prev)).skip(1).collect(), cost)
    }

    fn path(&self, start: char, goal: char) -> (String, usize) {
        let start_pos = self.map.find_first(|c| c == start).unwrap();
        let goal_pos = self.map.find_first(|c| c == goal).unwrap();

        let (path, cost) = astar(
                &State::new(start_pos, false, Move::Press),
                |n| {
                    let mut out = vec![];
                    if n.pos == goal_pos {
                        out.push((State::new(n.pos, true, Move::Press), 1));
                    } else {
                        Move::SHIFTS.into_iter()
                            .for_each(|m| {
                                let test = n.pos.advance(m);
                                if self.map.contains_key(&test) {
                                    out.push((State::new(test, false, m), 1));
                                }
                            });
                    }
                    out
                },
                |n| (n.pos - goal_pos).manhatan() as usize,
                |n| n.pressed
            ).unwrap();
        (path.into_iter().map(|s| char::from(s.prev)).skip(1).collect(), cost)
    }

}

type Costs = HashMap<(char, char), (String, usize)>;
struct Robot {
    keypad: Keypad,
    costs: Costs,
    pos: Vec2,
}

impl Robot {

    fn new(keypad: Keypad) -> Self {
        let mut costs = HashMap::new();
        for &a in keypad.map.values() {
            for &b in keypad.map.values() {
                let (path, _) = keypad.path(a, b);
                costs.insert((a, b), (path, 1));
            }
        }

        Robot{
            keypad,
            costs,
            pos: (2, 0).into()
        }
    }

    fn from_parrent(parrent: &Robot, keypad: Keypad) -> Self {
        let mut costs = HashMap::new();
        for &a in keypad.map.values() {
            for &b in keypad.map.values() {
                costs.insert((a, b), keypad.path_with_cost(a, b, &parrent.costs));
            }
        }

        Robot{
            keypad,
            costs,
            pos: (2, 0).into()
        }
    }

    fn press(&mut self, key: impl Into<Move>) -> Option<char> {
        let m = key.into();
        self.pos = self.pos.advance(m);
        if m == Move::Press {
            self.keypad.map.get(&self.pos).copied()
        } else {
            None
        }
    }

    fn encode(&self, line: &str) -> String {
        let mut pos = self.keypad.map.find_first(
            |c| c == 'A'
        ).unwrap();

        line.chars().filter_map(|c| {
            if c == 'A' {
                self.keypad.map.get(&pos)
            } else {
                pos = pos.advance(Move::from(c));
                None
            }
        }).collect()
    }

    fn decode(&self, line: &String, costs: &Costs) -> (String, usize) {
        let line = "A".to_string() + line;
        let mut out = String::new();
        let cost = line.chars()
            .tuple_windows()
            .map(|(a, b)| {
                let (path, c) = self.keypad.path_with_cost(a, b, costs);

                out += &path;
                c
            }).sum();
        (out, cost)
    }

    fn eval(&self, line: &String) -> usize {
        let line = "A".to_string() + line;
        line.chars()
            .tuple_windows()
            .map(|(a, b)| {
                self.costs.get(&(a, b)).unwrap().1
            }).sum()
    }
}

impl Debug for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = 3;
        writeln!(f, "Position: {}", self.pos)?;
        writeln!(f, "Costs:",)?;

        let keys = if self.keypad.map.len() > 5 {
            "A0123456789"
        } else {
            "A^>v<"
        };

        write!(f, " ")?;
        for b in keys.chars() {
            write!(f, "{:>width$}", b)?;
        }
        writeln!(f)?;

        for a in keys.chars() {
            write!(f, "{}", a)?;
            for b in keys.chars() {
                let v = self.costs.get(&(a, b)).unwrap().1;
                write!(f, "{v:>width$}")?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}


struct Robots{
    robots: Vec<Robot>
}

impl Robots {

    fn new(n: usize) -> Self {
        let mut robots = vec![Robot::new(Keypad::directional())];
        for i in 1..n {
            robots.push(Robot::from_parrent(
                &robots[i-1],
                Keypad::directional())
            );
        }

        robots.push(Robot::from_parrent(
            &robots[robots.len() - 1],
            Keypad::numeric())
        );

        Self{robots}
    }

    fn decode(&self, line: &str) -> (Vec<String>, usize) {
        let mut out = vec![line.to_owned()];
        for i in (1..self.robots.len()).rev() {

            let (p, _) = self.robots[i].decode(
                &out[out.len() - 1],
                &self.robots[i-1].costs
            );
            out.push(p);
        }
        let cost = out[out.len() - 1].len();
        (out, cost)
    }

    fn encode(&self, line: &str) -> Vec<String> {
        let mut out = vec![line.to_owned()];
        for i in 1..self.robots.len() {
            let p = self.robots[i].encode(out.last().unwrap().as_str());
            out.push(p);
        };
        out
    }


}

fn parse(input: &str) -> Vec<(&str, usize)> {
    input.lines()
        .map(|l| {
            (l, l[..(l.len() - 1)].parse().unwrap())
        }).collect()
}



fn part1(input: &str) -> usize {
    let input = parse(input);
    let robots = Robots::new(3);

    input.into_iter().map(|(l, n)| {
        let cost = robots.robots
            .last()
            .unwrap()
            .eval(&l.to_string());
        cost * n
    }).sum()

}

fn part2(input: &str) -> usize {

    let input = parse(input);
    let robots = Robots::new(26);

    input.into_iter().map(|(l, n)| {
        let cost = robots.robots
            .last()
            .unwrap()
            .eval(&l.to_string());
        cost * n
    }).sum()
}

#[cfg(test)]
mod tests {

    use super::*;
    use super::Move::*;


   // #[test]
   // fn day21_encode_test() {
   //     let robots = Robots::new(3);
   //     let levels = robots.encode("<vA<AA>>^AvAA<^A>A<v<A>>^AvA^A<vA>^A<v<A>^A>AAvA^A<v<A>A>^AAAvA<^A>A");
   //     for (i, level) in levels.iter().enumerate() {
   //         println!("{i}: {level}");
   //     }
   //     assert_eq!(levels[3], "029A");

   //     let levels = robots.encode("<v<A>^>AvA^A<v<A>^>AA<vA<A>^>AAvAA^<A>A<vA^>AA<A>A<vA<A>^>AAA<A>vA^A");
   //     for (i, level) in levels.iter().enumerate() {
   //         println!("{i}: {level}");
   //     }
   //     assert_eq!(levels[3], "379A");

   //     let levels = robots.encode("<v<A>>^AvA^A<vA<AA>>^AAvA<^A>AAvA^A<vA>^AA<A>A<v<A>A>^AAAvA<^A>A");
   //     assert_eq!(levels[3], "379A");
   // }

    #[test]
    fn day21_problem_test() {
        let robots = Robots::new(3);

        println!("{:?}", robots.robots[2]);

        let (p, c) = robots.robots[3].keypad
            .path_with_cost('3', '7', &robots.robots[2].costs);

        println!("{p} {c}");
        assert_eq!(c, 23);

    }


  //  #[test]
  //  fn day21_path_test() {
  //      let robots = Robots::new(3);
  //      println!("robot2: {:?} ", robots.robots[1]);
  //      println!("robot3: {:?} ", robots.robots[2]);


  //      let pad = Keypad::directional();
  //      pad.map.print(' ');
  //      assert_eq!(pad.path('A', '<').0, "v<<A");
  //  }

 //   #[test]
 //   fn day21_cost_test() {
 //       let mut pad = Keypad::new_directional();
 //       pad.update_costs(&pad.costs.clone());
 //       assert_eq!(pad.costs.get(&('>', '^')), Some(3).as_ref());
 //       pad.update_costs(&pad.costs.clone());
 //       assert_eq!(pad.costs.get(&('A', '<')), Some(10).as_ref());
 //       assert_eq!(pad.costs.get(&('<', 'A')), Some(8).as_ref());
 //       let mut pad2 = Keypad::new_numeric();
 //       pad2.update_costs(&pad.costs.clone());
 //       assert_eq!(pad2.get_cost("029A".to_string()), 68);
 //       assert_eq!(pad2.get_cost("980A".to_string()), 60);
 //       assert_eq!(pad2.get_cost("179A".to_string()), 68);
 //       assert_eq!(pad2.get_cost("456A".to_string()), 64);
 //       assert_eq!(pad2.get_cost("379A".to_string()), 64);
 //   }


  #[test]
  fn day21_part1_test() {
      let input = include_str!("../input_test.txt");
      assert_eq!(part1(input), 126384);
  }

  #[test]
  fn day21_part1_final_test() {
      let input = include_str!("../input.txt");
      assert_eq!(part1(input), 157892);
  }

  #[test]
  fn day21_part2_test() {
      let input = include_str!("../input_test.txt");
      assert_eq!(part2(input), 154115708116294);
  }

  #[test]
  fn day21_part2_final_test() {
      let input = include_str!("../input.txt");
      assert_eq!(part2(input), 197015606336332);
  }
}
