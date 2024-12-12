use std::collections::{HashSet, VecDeque};

use utils::{direction::{cardinal::Direction, AdvanceInDirection}, map::{Map, SparseMap}, run_task, took, vector::Vec2};

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input));
    run_task!(|| part2(input));
}

fn expand_plot(pos: Vec2<isize>, map: &SparseMap<char>) -> (HashSet<Vec2<isize>>, HashSet<(Vec2<isize>, Direction)>) {

    let plot =  map.get(&pos).unwrap();

    let mut gardens = HashSet::new();
    let mut queue = VecDeque::new();
    let mut fences  = HashSet::new();

    gardens.insert(pos);
    queue.push_back(pos);

    while let Some(n) = queue.pop_front() {
        Direction::ALL
            .into_iter()
            .for_each(|d| {
                let next = n.advance(&d);

                if !gardens.contains(&next) {
                    match map.get(&next) {
                        Some(v) if v == plot => {
                            gardens.insert(next);
                            queue.push_back(next);
                        }
                        _ => {
                            fences.insert((next, d));
                        }
                    }
                }
            });
    }
    (gardens, fences)
}

// 20 min
fn part1(input: &str) -> usize {
    let mut map = SparseMap::from_str(input, &|c| Some(c));

    let mut price = 0;
    while let Some(pos) = map.keys().next() {
        let (gardens, fences) = expand_plot(*pos, &map);

        price += gardens.len() * fences.len();

        for p in gardens {
            map.remove(&p);
        }
    };

    price
}


// 30 min
fn part2(input: &str) -> usize {
    let mut map = SparseMap::from_str(input, &|c| Some(c));

    let mut price = 0;
    while let Some(pos) = map.keys().next() {
        let (gardens, mut fences) = expand_plot(*pos, &map);

        let garden_size = gardens.len();

        for p in gardens {
            map.remove(&p);
        }

        let mut fence_segments = 0;
        while let Some((p, d)) = fences.iter().next() {
            let p = *p;
            let d = *d;
            fences.remove(&(p, d));

            let dd = d.turn_left();
            let mut pp = p.advance(&dd);
            while fences.contains(&(pp, d)) {
                fences.remove(&(pp, d));
                pp = pp.advance(&dd);
            };

            let dd = d.turn_right();
            let mut pp = p.advance(&dd);
            while fences.contains(&(pp, d)) {
                fences.remove(&(pp, d));
                pp = pp.advance(&dd);
            };

            fence_segments += 1;

        }


        price += garden_size * fence_segments;
    };

    price
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day12_part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 1930);
    }

    #[test]
    fn day12_part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 1464678);
    }

    #[test]
    fn day12_part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 1206);
    }

    #[test]
    fn day12_part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 877492);
    }
}
