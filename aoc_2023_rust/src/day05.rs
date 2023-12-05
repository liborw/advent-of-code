use std::fmt::Debug;
use itertools::Itertools;

trait Mapping: Debug {
    fn map_source_to_destionation(&self, source: usize) -> Option<usize>;
    fn map_interval(&self, source: &Interval) -> (Vec<Interval>, Vec<Interval>);
}

#[derive(Debug)]
struct Range {
    destination_range_start: usize,
    source_range_start: usize,
    range_length: usize
}

impl Range {
    fn new(destination_range_start: usize, source_range_start: usize, range_length: usize) -> Self {
        Range{destination_range_start, source_range_start, range_length}
    }

    fn source_interval(&self) -> Interval {
        return Interval(self.source_range_start, self.range_length)
    }
}

impl Mapping for Range {
    fn map_source_to_destionation(&self, source: usize) -> Option<usize> {
        source.checked_sub(self.source_range_start).and_then(|v| {
            if v < self.range_length {
                Some(v + self.destination_range_start)
            } else {
                None
            }
        })
    }

    fn map_interval(&self, a: &Interval) -> (Vec<Interval>, Vec<Interval>) {
        let b = self.source_interval();
        if (b.start() > a.end()) || (a.start() > b.end()) {
            (Vec::new(), vec![a.clone()])
        } else {
            let os = a.start().max(b.start());
            let oe = a.end().min(b.end());
            let s = self.destination_range_start + (os - b.start());

            let mut rem = Vec::new();
            if  os > a.start() {
                rem.push(Interval(a.start(), os - a.start()));
            }

            if oe < a.end() {
                rem.push(Interval(oe, a.end() - oe));
            }


            (vec![Interval(s, oe - os)], rem)
        }
    }
}

#[derive(Debug)]
struct IdentityRange();

impl Mapping for IdentityRange {
    fn map_source_to_destionation(&self, source: usize) -> Option<usize> { Some(source) }
    fn map_interval(&self, source: &Interval) -> (Vec<Interval>, Vec<Interval>) { (vec![source.clone()], Vec::new()) }
}

#[derive(Debug)]
struct Map {
    ranges: Vec<Box<dyn Mapping>>
}

impl Mapping for Map {
    fn map_source_to_destionation(&self, source: usize) -> Option<usize> {
        self.ranges.iter().find_map(|m| m.map_source_to_destionation(source))
    }
    fn map_interval(&self, a: &Interval) -> (Vec<Interval>, Vec<Interval>) {
        let mut rem = vec![a.clone()];
        let mut out = Vec::new();
        for range in self.ranges.iter() {
            let mut new_rem = Vec::new();
            for r in rem.into_iter() {
                let (out_part, rem_part) = range.map_interval(&r);
                out.extend(out_part);
                new_rem.extend(rem_part)
            }
            rem = new_rem;
        }
        (out, Vec::new())
    }
}

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Interval(usize, usize);

impl Interval {
    fn start(&self) -> usize { self.0 }
    fn end(&self) -> usize { self.0 + self.1 }
    fn len(&self) -> usize { self.1 }
}


fn input() -> (Vec<usize>, Vec<Map>) {
    let mut blocks = include_str!("./input/day05.txt").split("\n\n");
    let seed_block = blocks.next().unwrap();

    let seeds: Vec<usize> = seed_block.split_once(": ")
        .map(|(_, s)| {
             s.split_whitespace().map(|v| v.parse::<usize>().unwrap()).collect()
        }).unwrap();

    let maps = blocks.into_iter().map(|s| {
         let (_, ranges_str) = s.split_once("\n").unwrap();
         let mut ranges: Vec<Box<dyn Mapping>> = Vec::new();
         ranges_str.lines()
             .for_each(|s| {
                 let (a, b, c) = s.split_whitespace().map(|s| s.parse::<usize>().unwrap()).collect_tuple().unwrap();
                 ranges.push(Box::new(Range::new(a, b, c)))
             });
         ranges.push(Box::new(IdentityRange()));
         Map{ranges}
    }).collect();
    (seeds, maps)
}

pub fn day05a() -> usize {
    let (seeds, maps) = input();
    seeds.into_iter()
         .map(|s| {
             maps.iter().fold(s, |v, map| map.map_source_to_destionation(v).unwrap())
         }).min().unwrap()
}

pub fn day05b() -> usize {
    let (seeds, maps) = input();
    let intervals: Vec<Interval> = seeds.chunks(2).map(|chunk| Interval(chunk[0], chunk[1])).collect();
    let v = intervals.into_iter()
         .map(|i| {
             maps.iter().fold(vec![i], |v, map| {
                 v.into_iter().map(|i| map.map_interval(&i).0.into_iter()).flatten().unique().collect::<Vec<Interval>>()
             }).into_iter().map(|i| i.start()).min().unwrap()
         }).min().unwrap();

    v
}
