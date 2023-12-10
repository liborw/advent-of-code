use took::took;
use common::pos::*;
use common::map::{Map, SparseMap};

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
    input.lines().enumerate()
        .map(|(i, l)| {
            l.chars().enumerate().filter_map(move |(j, ch)| {
                if ch != '.' {
                    Some(((i as isize, j as isize).into(), ch))
                } else {
                    None
                }
            })
        }).flatten().collect()
}



fn expand(map: &SparseMap<char>, pos: &Pos) -> Vec<Pos> {
    match map.get(pos).unwrap_or(&'.').to_owned() {
        '|' => vec![*pos + (-1,  0).into(), *pos + ( 1,  0).into()],
        '-' => vec![*pos + ( 0,  1).into(), *pos + ( 0, -1).into()],
        'F' => vec![*pos + ( 0,  1).into(), *pos + ( 1,  0).into()],
        'L' => vec![*pos + (-1,  0).into(), *pos + ( 0,  1).into()],
        'J' => vec![*pos + (-1,  0).into(), *pos + ( 0, -1).into()],
        '7' => vec![*pos + ( 0, -1).into(), *pos + ( 1,  0).into()],
        'S' => {
            pos.neighbors4().into_iter().filter_map(|p| {
                if expand(map, &p).contains(pos) {
                    Some(p)
                } else {
                    None
                }
            }).collect()
        }
        '.' => vec![],
         v => unreachable!("{}", v)
    }
}


fn part1(input: &str) -> usize {
    let map = parse(input);
    let mut pos = map.iter().find(|(_, v)| v == &&'S').unwrap().0.to_owned();
    let mut n = 0;
    let mut visited = vec![pos];

    loop {
        n += 1;
        //println!("{pos:?} {visited:?}");
        if let Some(v) = expand(&map, &pos).into_iter().find_map(|p| {
            if !visited.contains(&p) {
                Some(p)
            } else {
                None
            }
        }) {
            visited.push(v);
            pos = v;
        } else {
            break
        };
    }
    n / 2
}


fn part2(input: &str) -> usize {
    let map = parse(input);
    let start = map.iter().find(|(_, v)| v == &&'S').unwrap().0.to_owned();
    let mut pos = start;
    let mut visited = vec![pos];

    loop {
        //println!("{pos:?} {visited:?}");
        if let Some(v) = expand(&map, &pos).into_iter().find_map(|p| {
            if !visited.contains(&p) {
                Some(p)
            } else {
                None
            }
        }) {
            visited.push(v);
            pos = v;
        } else {
            break
        };
    }

    // outside inside
    let mut oimap = SparseMap::new();
    visited.push(visited[0]);
    visited.push(visited[1]);
    visited.windows(3).for_each(|w| {

        oimap.insert(w[1], '#');
        w[1].neighbors4().iter().cycle()
            .skip_while(|p| p != &&w[0])
            .take_while(|p| p != &&w[2]).for_each(|p| {
                oimap.entry(*p).or_insert('O');
        });

        w[1].neighbors4().iter().cycle()
            .skip_while(|p| p != &&w[2])
            .take_while(|p| p != &&w[0]).for_each(|p| {
                oimap.entry(*p).or_insert('I');
        });
    });


    // What is inside, can't be outside
    let bb = oimap.bounding_box();
    let mut inside = (bb.y_min..bb.y_max).into_iter().find_map(|y| oimap.get(&(y, bb.x_min).into())).unwrap().clone();

    if inside == 'I' {
        inside = 'O'
    } else {
        inside = 'I'
    };

    // Grow arreas
    oimap.clone().iter().filter(|(_, v)| v == &&inside).for_each(|(pos, _)| {

        let mut p = *pos;
        loop {
            p = p + (0, -1).into();
            if oimap.contains_key(&p) || p.y < bb.y_min {
                break
            } else {
                oimap.insert(p, inside.clone());
            }
        }

        let mut p = *pos;
        loop {
            p = p + (0, 1).into();
            if oimap.contains_key(&p) || p.y > bb.y_max {
                break
            } else {
                oimap.insert(p, inside.clone());
            }
        }
    });

    oimap.dump('.');
    oimap.values().filter(|v|v == &&inside).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test1() {
        let input = include_str!("../input_test1.txt");
        assert_eq!(part1(input), 4);
    }

    #[test]
    fn part1_test_expand_start() {
        let input = include_str!("../input_test1.txt");
        let map = parse(input);
        let pos = map.iter().find(|(_, v)| v == &&'S').unwrap().0.to_owned();
        assert_eq!(expand(&map, &pos), vec![(2,1).into(), (1,2).into()]);
    }

    #[test]
    fn part1_test2() {
        let input = include_str!("../input_test2.txt");
        assert_eq!(part1(input), 8);
    }

   #[test]
   fn part1_final_test() {
       let input = include_str!("../input.txt");
       assert_eq!(part1(input), 6870);
   }

    #[test]
    fn part2_test() {

        let input = "
...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
...........";

        assert_eq!(part2(input), 4);
    }
}
