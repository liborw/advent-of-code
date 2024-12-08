use std::{env, fmt::Display, fs};


const NAMES: &str = "123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ";

fn main() {
    let args: Vec<String> = env::args().collect();
    let input = fs::read_to_string(args[1].clone()).unwrap();
    part1(input.as_str());
}


#[derive(Debug, Clone)]
struct Car {
    name: char,
    tempo: i32,
    length: i32,
    pos: i32,
    lane: i32
}

impl Car {
    fn new(name: char, tempo: i32, length: i32, pos: i32, lane: i32) -> Self {
        Car{name, tempo, length, pos, lane}
    }

    fn will_fit(&self, other: &Car) -> bool {
        if other.name == self.name {
            return true
        }

        if other.lane != self.lane {
            return true
        }

        if (self.pos - self.length < other.pos) && (other.pos <= self.pos) {
            return false
        }

        if (other.pos - other.length < self.pos) && (self.pos <= other.pos) {
            return false
        }

        true
    }

    fn range(&self) -> Vec<i32> {
        let from = self.pos - self.length + 1;
        (from..=self.pos).collect()
    }

}


struct Simulation {
    finish: i32,
    lanes: i32,
    cars: Vec<Car>,
}

impl Simulation {
    fn step(&mut self, tick: i32) -> bool {
        let mut change = false;
        for i in 0..self.cars.len() {
            if tick % self.cars[i].tempo != 0 {
                continue
            }

            let mut test = self.cars[i].clone();
            test.pos += 1;
            if self.is_valid(&test) {

                test.lane -= 1;
                if !self.is_valid(&test) {
                    test.lane += 1;
                }
                self.cars[i] = test;
                change = true;
                continue;
            }

            let mut test = self.cars[i].clone();
            test.pos += 1;
            if self.is_valid(&test) {
                self.cars[i] = test;
                change = true;
                continue;
            }

            let mut test = self.cars[i].clone();
            test.lane += 1;
            if self.is_valid(&test) {
                self.cars[i] = test;
                change = true;
                continue;
            }
        }
        change
    }

    fn is_valid(&self, car: &Car) -> bool {
        if car.lane >= self.lanes || car.lane < 0 {
            return false
        }
        self.cars.iter().all(|c| c.will_fit(car))
    }

}

impl Display for Simulation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {

        let l = self.finish + 5 ;
        let mut img = vec![vec!['.'; l as usize]; self.lanes as usize];

        for i in 0..self.lanes {
            img[i as usize][(self.finish) as usize] = '|';
        }

        for car in self.cars.iter() {
            for i in car.range() {
                if i >= 0 && i < l {
                    img[car.lane as usize][i as usize] = car.name;
                }
            }
        }

        img.reverse();
        for line in img {
            writeln!(f, "{}", line.iter().collect::<String>())?;
        }
        Ok(())
    }
}


fn parse(input: &str) -> Simulation {
    let mut lines = input.lines();
    let finish = lines.next().unwrap().parse().unwrap();
    let lanes = lines.next().unwrap().parse().unwrap();
    let names: Vec<_> = NAMES.chars().collect();
    let cars = lines.enumerate().map(|(i, l)| {
        let mut parts = l.split_whitespace().map(|v| v.parse().unwrap());

        Car::new(
            names[i],
            parts.next().unwrap(),
            parts.next().unwrap(),
            parts.next().unwrap(),
            parts.next().unwrap(),
        )

    }).collect();

    Simulation{finish, lanes, cars}
}


fn part1(input: &str) -> i32 {
    let mut sim = parse(input);
    let mut first = None;
    let mut last = None;

    print!("init:\n{}", sim);

    for i in 0.. {
        if sim.step(i) {
            print!("time: {}\n{}", i, sim);
            // check for first to the finish line
            if first.is_none() && sim.cars.iter().any(|c| c.pos >= sim.finish) {
                first = Some(i);
            }

            // check for last to the finish line
            let car  = sim.cars.iter().min_by_key(|c| c.pos).unwrap();
            if car.pos >= sim.finish {
                last = Some(i);
                break
            }
        }
    }

    let diff = last.unwrap() - first.unwrap();
    println!("First: {} Last: {} Diff: {}", first.unwrap(), last.unwrap(), diff);
    diff
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 45);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 4328);
    }

}
