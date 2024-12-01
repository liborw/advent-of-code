use std::{collections::HashMap, str::FromStr, hash::Hasher, fmt::{Display, format}};

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

trait Apply {
    fn apply(&self, part: Part) -> Option<Outcome>;
}

type Part = [usize; 4];

#[derive(Debug, Clone)]
enum Outcome {
    Accept,
    Reject,
    Next(String),
}

impl Outcome {
    fn is_next(&self) -> bool {
        match self {
            Outcome::Next(_) => true,
            _ => false
        }
    }

    fn next(&self) -> Option<String> {
        match self {
            Outcome::Next(v) => Some(v.to_string()),
            _ => None
        }
    }
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Outcome::*;
        match s {
            "A" => Ok(Accept),
            "R" => Ok(Reject),
             v  => Ok(Next(v.to_string())),
        }
    }
}

#[derive(Debug)]
enum Rule {
    More(Outcome, usize, usize),
    Less(Outcome, usize, usize),
    Default(Outcome)
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Rule::*;

        if s.contains(":") {
            let (rule, next) = s.split_once(':').unwrap();
            let outcome = Outcome::from_str(next).unwrap();

            let (left, value) = rule.split_at(2);
            let (cat, ord) = left.split_at(1);

            let cat = match cat {
                "x" => 0,
                "m" => 1,
                "a" => 2,
                "s" => 3,
                 v  => panic!("Unknow category: {v}")

            };

            match ord {
                ">" => Ok(More(outcome, cat, value.parse().unwrap())),
                "<" => Ok(Less(outcome, cat, value.parse().unwrap())),
                 _  => panic!(),
            }


        } else {
            Ok(Default(Outcome::from_str(s).unwrap()))
        }
    }
}

impl Rule {
    fn apply(&self, part: Part) -> Option<Outcome> {
        use Rule::*;
        match self {
            More(o, cat, val) => {
                if part[*cat] > *val {
                    Some(o.clone())
                } else {
                    None
                }
            },
            Less(o, cat, val) => {
                if part[*cat] < *val {
                    Some(o.clone())
                } else {
                    None
                }
            },
            Default(o) => Some(o.clone())
        }

    }
}

#[derive(Debug)]
struct Workflow {
    rules: Vec<Rule>,
}

impl Apply for Workflow {
    fn apply(&self, part: Part) -> Option<Outcome> {
        self.rules.iter().find_map(|r| r.apply(part))
    }
}

impl Apply for HashMap<String, Workflow> {
    fn apply(&self, part: Part) -> Option<Outcome> {
        let mut outcome = Outcome::Next("in".to_string());

        while outcome.is_next() {
            outcome = self.get(&outcome.next().unwrap()).unwrap().apply(part).unwrap();
        }

        Some(outcome)
    }
}



fn parse(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let (workflows, parts) = input.split_once("\n\n").unwrap();

    let workflows = workflows.lines().map(|l| {
        let (lbl, rest) = l.split_once('{').unwrap();
        let rest = rest.strip_suffix("}").unwrap();
        let rules = rest.split(',').map(|s| Rule::from_str(s).unwrap()).collect();
        (lbl.to_string(), Workflow{rules})
    }).collect();

    let parts = parts.lines().map(|l| {
        let vec = l.split('=').skip(1).map(|g| {
            g.chars().take_while(|c| c.is_numeric()).collect::<String>()
        }).map(|s| s.parse::<usize>().unwrap()).collect::<Vec<usize>>();
        [vec[0], vec[1], vec[2], vec[3]]
    }).collect();

    (workflows, parts)
}

fn part1(input: &str) -> usize {
    let (wfs, parts) = parse(input);
    parts.iter().map(|p| {

        match wfs.apply(*p).unwrap() {
            Outcome::Accept => {
                p.iter().sum()
            },
            _ => 0
        }
    }).sum()
}

const MAX: usize = 4000;
const MIN: usize = 1;

#[derive(Debug, PartialEq, Clone, Copy)]
enum Range {
    Empty,
    Bounds(usize, usize)
}

impl Default for Range {
    fn default() -> Self {
        Range::Bounds(MIN, MAX)
    }
}

impl Range {
    fn new(start: usize, end: usize) -> Self {
        Range::Bounds(start, end)
    }

    fn merge(&mut self, other: &Range) {
        use Range::*;

        *self = match (*self, *other) {
            (Empty, _) => Empty,
            (_, Empty) => Empty,
            (Bounds(l0, u0), Bounds(l1, u1)) => {
                if l0 < u1 && l1 < u0 {
                    Bounds(l0.max(l1), u0.min(u1))
                } else {
                    Empty
                }
            },
        };
    }

    fn size(&self) -> usize {
        use Range::*;
        match self {
            Empty => 0,
            Bounds(l, u) => u - l + 1
        }
    }
}


impl Display for Range {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Range::*;
        match self {
            Empty => write!(f, "[]"),
            Bounds(l, u) => write!(f, "[{l}, {u}]"),
        }
    }
}


fn solve(wfs: &HashMap<String, Workflow>, agg: HashMap<usize, Range>, outcome: &Outcome, hist: String) -> usize {

    match outcome {
        Outcome::Reject => 0,
        Outcome::Next(next) => {
            let mut agg_next = agg.clone();
            wfs.get(next).unwrap().rules.iter().map(|r| {
                match r {
                    Rule::Less(out, cat, v) => {
                        let mut agg_pos = agg_next.clone();
                        agg_pos.entry(*cat).or_default().merge(&Range::new(MIN, v - 1));
                        agg_next.entry(*cat).or_default().merge(&Range::new(*v, MAX));
                        solve(wfs, agg_pos, out, format!("{hist}->{next}"))
                    },
                    Rule::More(out, cat, v) => {
                        let mut agg_pos = agg_next.clone();
                        agg_pos.entry(*cat).or_default().merge(&Range::new(v + 1, MAX));
                        agg_next.entry(*cat).or_default().merge(&Range::new(MIN, *v));
                        solve(wfs, agg_pos, out, format!("{hist}->{next}"))
                    }
                    Rule::Default(out) => {
                        solve(wfs, agg_next.clone(), out, format!("{hist}->{next}"))
                    }
                }
            }).sum()
        },
        Outcome::Accept => {
            //println!("{hist}");
            ['x', 'm', 'a', 's'].into_iter().enumerate().map(|(i, key)| {
                let mut agg = agg.clone();
                let r = agg.entry(i).or_default();
                let s = r.size();
                //println!("{key}: {r} {s}");
                s
            }).product()
        }
    }
}


fn part2(input: &str) -> usize {
    let (wfs, _) = parse(input);
    solve(&wfs, HashMap::new(), &Outcome::Next("in".to_string()), "".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 19114);
    }

    #[test]
    fn part1_final_test() {
        let input = include_str!("../input.txt");
        assert_eq!(part1(input), 476889);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 167409079868000);
    }

    // #[test]
    // fn part2_final_test() {
    //     let input = include_str!("../input.txt");
    //     assert_eq!(part2(input), 1);
    // }
}
