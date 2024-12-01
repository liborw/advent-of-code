use std::collections::{HashMap, VecDeque};

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

#[derive(Debug, Clone, PartialEq, Eq)]
enum Module {
    Broadcaster,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
    Detector(bool),
}

impl Module {
    fn receive(&mut self, sig: bool, from: &str) -> Option<bool> {
        use Module::*;
        match (self, sig) {
            (Broadcaster, s) => Some(s),
            (FlipFlop(_), true) => None,
            (FlipFlop(ref mut v), false) => {
                *v = !*v;
                Some(*v)
            }
            (Conjunction(ref mut map), s) => {
                map.insert(from.to_string(), s);
                Some(!map.values().all(|v| *v))
            }
            (Detector(ref mut v), false) => {
                *v = true;
                None
            }
            (Detector(_), true) => None
        }
    }
}

type Modules = HashMap<String, (Module, Vec<String>)>;

fn parse(input: &str) -> Modules {
    use Module::*;
    let mut inputs: HashMap<String, Vec<String>> = HashMap::new();
    let mut modules: Modules = input.lines().map(|l| {
        let (label, output) = l.split_once(" -> ").unwrap();
        let output: Vec<String> = output.split(", ").map(|s| s.to_string()).collect();

        let (label, module) = match label {
            s if s.starts_with("&") => (label[1..].to_string(), (Conjunction(HashMap::new()), output)),
            s if s.starts_with("%") => (label[1..].to_string(), (FlipFlop(false), output)),
            "broadcaster"  => (label.to_string(), (Broadcaster, output)),
            s => unreachable!("Found: {s}")
        };

        for out in module.1.iter() {
            inputs.entry(out.to_string()).or_insert(Vec::new()).push(label.to_string());
        }

        (label, module)
    }).collect();

    inputs.iter().for_each(|(key, inputs)| {
        if let Some((Conjunction(m), _)) = modules.get_mut(key) {
            for i in inputs {
                m.insert(i.to_string(), false);
            }
        }
    });
    modules
}

type Message = (String, bool, String);

fn pulse(modules: &mut Modules, sig: bool) -> (usize, usize) {
    let mut queue: VecDeque<Message> = VecDeque::new();
    queue.push_back(("button".to_string(), sig, "broadcaster".to_string()));
    let mut high_cnt = 0;
    let mut low_cnt = 0;


    while !queue.is_empty() {
        let msg = queue.pop_front().unwrap();
        //println!("{} -({})-> {}", msg.0, msg.1, msg.2);

        if msg.1 {
            high_cnt += 1;
        } else {
            low_cnt += 1;
        }

        if let Some((ref mut module, outputs)) = modules.get_mut(&msg.2) {
            if let Some(v) = module.receive(msg.1, &msg.0) {
                outputs.iter().for_each(|o| {
                    queue.push_back((msg.2.clone(), v, o.to_string()));
                })
            }
        }
    };

    (high_cnt, low_cnt)

}

fn part1(input: &str) -> usize {
    let mut modules = parse(input);
    let (h, l) = (0..1000).map(|_| pulse(&mut modules, false)).fold((0, 0), |(hh, ll), (h, l)| (hh+h, ll+l));
    h*l
}

fn part2(input: &str) -> usize {
    let mut modules = dbg!(parse(input));
    modules.insert("rx".to_string(), (Module::Detector(false), vec![]));


    (0usize..).find(|_| {
        pulse(&mut modules, false);
        Module::Detector(true) == modules.get("rx").unwrap().0
    }).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 32000000);
    }

    #[test]
    fn part1_test2() {
        let input = include_str!("../input_test2.txt");
        assert_eq!(part1(input), 11687500);
    }

    #[test]
    fn flipflop_test() {
        let mut ff = Module::FlipFlop(false);
        assert_eq!(ff.receive(true, "a"), None);
        assert_eq!(ff, Module::FlipFlop(false));
        assert_eq!(ff.receive(false, "a"), Some(true));
        assert_eq!(ff, Module::FlipFlop(true));
    }

    #[test]
    fn conjunction_test() {
        let mut c = Module::Conjunction([("s".to_string(), false)].into_iter().collect());
        assert_eq!(c.receive(false, "s"), Some(true));
        assert_eq!(c.receive(true, "s"), Some(false));
        assert_eq!(c.receive(false, "s"), Some(true));
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 1020211150);
    }

    #[test]
    fn part2_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part2(input), 1);
    }
}
