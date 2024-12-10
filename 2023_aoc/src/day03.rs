use itertools::Itertools;
use common::map::*;

type EngineSchematic = SparseMap<char>;

fn input() -> EngineSchematic {
    let mut map: EngineSchematic = EngineSchematic::new();
    include_str!("./input/day03.txt")
        .lines()
        .enumerate()
        .for_each(|(r, l)| {
            l.chars().enumerate().for_each(|(c, ch)| {
                if ch != '.' {
                    map.insert(Position::new(r as isize, c as isize), ch);
                }
            })
        });
    map
}

fn grow_number(map: &EngineSchematic, pos: &Position) -> (isize, isize, u32) {
    let mut col = pos.col;
    loop {
        if !map.get(&Position::new(pos.row, col - 1)).is_some_and(|c| c.is_digit(10)) {
            break
        }
        col -= 1;
    }

    let col_start = col;

    let mut digits = Vec::new();
    loop {
        if let Some(v) = map.get(&Position::new(pos.row, col)).and_then(|v| v.to_digit(10)) {
            col += 1;
            digits.push(v)
        } else {
            break
        }
    }

    (pos.row, col_start, digits.into_iter().fold(0, |a, v| a * 10 + v))
}

pub fn day03a() -> u32 {
    let map = input();
    let bb = map.bounding_box();
    let mut part_numbers: Vec<u32> = Vec::new();
    for row in bb.row_min..=bb.row_max {
        let mut number = 0;
        let mut add = false;
        for col in bb.col_min..=(bb.col_max + 1) {
            let p = Position::new(row, col);
            if let Some(n) = map.get(&p).map(|v| v.to_digit(10)).flatten() {
                number = number * 10 + n;
                if p.neightbours(Adjacency::Eigth).iter().any(|n| map.get(&n).is_some_and(|v| !v.is_digit(10))) {
                    add = true;
                }
            } else {
                if add {
                    part_numbers.push(number);
                }
                number = 0;
                add = false;
            }
        }
    }
    part_numbers.into_iter().sum()
}

pub fn day03b() -> u32 {
    let map = input();

    map.iter()
        .filter(|(_, v)| v == &&'*')
        .filter_map(|(p, _)| {
            let parts: Vec<Position> = p.neightbours(Adjacency::Eigth).into_iter()
                         .filter(|n| map.get(n).is_some_and(|c| c.is_digit(10)))
                         .collect();

            let parts:Vec<_> = parts.into_iter()
                     .map(|p| grow_number(&map, &p))
                     .unique()
                     .map(|v| v.2)
                     .collect();

            if parts.len() == 2 {
                parts.into_iter().reduce(|acc, v| acc * v)
            } else {
                None
            }
        }).sum()
}
