use std::{str::FromStr, num::ParseIntError};

#[derive(Debug, PartialEq, PartialOrd, Default)]
struct Boxes {
    red: u32,
    green: u32,
    blue: u32
}

impl Boxes {

    fn possible(&self, bag: &Boxes) -> bool {
        (self.red <= bag.red) && (self.green <= bag.green) && (self.blue <= bag.blue)
    }
}

impl FromStr for Boxes {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut boxes = Boxes::default();
        s.split(",")
         .map(|s| s.trim().split_once(" ").unwrap())
         .for_each(|(v, b)| {
             let v: u32 = v.parse().unwrap();
             match b {
                 "red" => boxes.red += v,
                 "green" => boxes.green += v,
                 "blue" => boxes.blue += v,
                 _ => ()
             };
         });
        Ok(boxes)
    }
}


#[derive(Debug)]
struct Game {
    id: usize,
    draws: Vec<Boxes>
}

impl Game {
    fn possible(&self, bag: &Boxes) -> bool {
        self.draws.iter().all(|d| d.possible(bag))
    }

    fn min_bag(&self) -> Boxes {
        let mut bag = Boxes::default();
        self.draws.iter().for_each(|b| {
            if bag.red < b.red {
                bag.red = b.red;
            }

            if bag.green < b.green {
                bag.green = b.green;
            }

            if bag.blue < b.blue {
                bag.blue = b.blue;
            }
        });
        bag
    }

    fn power(&self) -> u32 {
        let bag = self.min_bag();
        bag.red * bag.green * bag.blue
    }
}

impl FromStr for Game {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, draws) = s.split_once(":").unwrap();
        let id: usize = game.split_once(" ").map(|v| v.1.parse().unwrap()).unwrap();
        let draws: Vec<Boxes> = draws.split(";").map(|d| d.parse().unwrap()).collect();
        Ok(Game{ id, draws })
    }
}

fn input() -> Vec<Game> {
    include_str!("./input/day02.txt").lines().map(|l| l.parse().unwrap()).collect()
}

pub fn day02a() -> usize {
    let bag = Boxes{red: 12, green: 13, blue: 14};
    input().iter().filter_map(|g| g.possible(&bag).then(|| g.id)).sum()
}

pub fn day02b() -> u32 {
    input().iter().map(|g| g.power()).sum()
}


