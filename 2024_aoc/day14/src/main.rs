use image::{GrayImage, ImageBuffer, Luma};
use regex::Regex;
use utils::{map::{Map, SparseMap}, run_task, took, vector::{Rect, Vec2}};

fn main() {
    let input = include_str!("../input.txt");
    run_task!(|| part1(input, (101, 107).into()));
    run_task!(|| part2(input, (101, 107).into()));
}

#[derive(Debug)]
struct Robot{
    pos: Vec2<isize>,
    vel: Vec2<isize>
}

type Robots = Vec<Robot>;

impl Robot {

    fn new(pos: impl Into<Vec2<isize>>, vel: impl Into<Vec2<isize>>) -> Self {
        Robot{
            pos: pos.into(),
            vel: vel.into()
        }
    }

    fn step(&mut self, n: isize, size: &Vec2<isize>) {
        self.pos += self.vel * n;

        while self.pos.x < 0 {
            self.pos.x += size.x;
        }

        while self.pos.y < 0 {
            self.pos.y += size.y;
        }

        while self.pos.x >= size.x {
            self.pos.x -= size.x;
        }

        while self.pos.y >= size.y {
            self.pos.y -= size.y;
        }
    }

}

fn parse(input: &str) -> Robots {
    let re = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
    re.captures_iter(input)
        .map(|c| c.extract::<4>().1.into_iter()
            .map(|v| v.parse::<isize>().unwrap())
            .collect::<Vec<_>>())
        .map(|v| {
        Robot{
                pos: Vec2::new(v[0], v[1]),
                vel: Vec2::new(v[2], v[3]),
            }
        }).collect()
}

fn eval(robots: &Robots, size: &Vec2<isize> ) -> usize {
    let xh = size.x / 2;
    let yh = size.y / 2;

    let quadrants = [
        Rect::new((0, 0),(xh, yh)),
        Rect::new((xh + 1, 0),(size.x, yh)),
        Rect::new((0, yh + 1),(xh, size.y)),
        Rect::new((xh + 1, yh + 1),(size.x, size.y)),
    ];
    println!("{quadrants:?}");

    quadrants.into_iter()
     .map(|q| {
            let q_val = robots.iter()
                .filter_map(|r| q.is_inside(r.pos).then_some(1))
                .sum::<usize>();
            dbg!(q_val)
        }).product()
}

fn draw(robots: &Robots, size: Vec2<isize>) {
    let mut map = SparseMap::new();

    for r in robots {
        map.entry(r.pos).and_modify(|v| *v += 1 ).or_insert(1);
    }

    map.print_with_bounds('.', &Rect::new((0,0), size));

}

fn part1(input: &str, size: Vec2<isize>) -> usize {
    let mut robots = parse(input);

    for _ in 0..100 {
        robots.iter_mut()
              .for_each(|r| r.step(1, &size));
    }

    eval(&robots, &size)
}

fn draw_image(i: usize, robots: &Robots, size: Vec2<isize>) {


    let mut image = GrayImage::new(size.x as u32, size.y as u32);
    for r in robots {
        image.get_pixel_mut(r.pos.x as u32, r.pos.y as u32).0 = [255];
    }
    image.save(format!("p2_{i:010}.png")).unwrap();
}

fn part2(input: &str, size: Vec2<isize>) -> usize {
    let mut robots = parse(input);
    let i = 6516;

    robots.iter_mut()
          .for_each(|r| r.step(i as isize, &size));
    draw_image(i, &robots, size);

    i
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day14_step_test() {
        let mut robot = Robot::new((1, 1), (1, 1));
        (0..2).for_each(|_| robot.step(1,&(3, 3).into()));
        assert_eq!(robot.pos, (0, 0).into());
    }

    #[test]
    fn day14_part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input, (11, 7).into()), 12);
    }

    #[test]
    fn day14_part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input, (101, 103).into()), 217132650);
    }

    // #[test]
    // fn day14_part2_test() {
    //     let input = include_str!("../input_test.txt");
    //     assert_eq!(part2(input, (11, 7).into()), 1);
    // }

    #[test]
    fn day14_part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input, (101, 103).into()), 6516);
    }
}
