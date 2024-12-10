use std::{cmp::Ordering, collections::HashMap, num::ParseIntError};

use itertools::Itertools;
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

fn parse(input: &str) -> Vec<Hand> {
    input.lines().into_iter().map(|l| {
        l.try_into().unwrap()
    }).collect()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfaKind,
    FullHouse,
    FourOfaKind,
    FiveOfaKind,
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: String,
    bid: usize,
    htype: HandType
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.htype.cmp(&other.htype) {
            Ordering::Equal => {
                Some(self.cards.cmp(&other.cards))
            }
            d => Some(d)
        }
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<'a> TryFrom<&'a str> for Hand {
    type Error = ParseIntError;
    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let (cards, bid) = value.split_once(" ").unwrap();

        let mut map: HashMap<char, u32> = HashMap::new();
        cards.chars().for_each(|c| {
            map.entry(c).and_modify(|counter| *counter += 1).or_insert(1);
        });

        let htype = match map.values().sorted().rev().tuples().next() {
            None => HandType::FiveOfaKind,
            Some((4, 1)) => HandType::FourOfaKind,
            Some((3, 2)) => HandType::FullHouse,
            Some((3, 1)) => HandType::ThreeOfaKind,
            Some((2, 2)) => HandType::TwoPair,
            Some((2, 1)) => HandType::OnePair,
            Some((1, 1)) => HandType::HighCard,
            _ => unreachable!()
        };

        let map = HashMap::from([('A', 'm'),
                                 ('K', 'l'),
                                 ('Q', 'k'),
                                 ('J', 'j'),
                                 ('T', 'i'),
                                 ('9', 'h'),
                                 ('8', 'g'),
                                 ('7', 'f'),
                                 ('6', 'e'),
                                 ('5', 'd'),
                                 ('4', 'c'),
                                 ('3', 'b'),
                                 ('2', 'a')]);
        let cards:String = cards.chars().map(|c| map.get(&c).unwrap()).collect();

        Ok(Hand{
            cards,
            bid: bid.parse()?,
            htype
        })
    }
}

fn part1(input: &str) -> usize {
    parse(input).into_iter().sorted().enumerate().map(|(i, h)| (i+1) * h.bid).sum()
}


fn part2(input: &str) -> usize {
    // fix for the new rules
    let game: Vec<Hand> = parse(input).into_iter().map(|h| {

        let mut map: HashMap<char, u32> = HashMap::new();
        h.cards.chars().for_each(|c| {
            map.entry(c).and_modify(|counter| *counter += 1).or_insert(1);
        });
        let mut htype = h.htype.clone();
        if let Some(joker_v) = map.remove(&'j') {
            if let Some((k, _)) = map.iter().sorted_by_key(|(_, v)| *v).rev().next() {
                map.entry(*k).and_modify(|v| *v += joker_v);
                htype = match map.values().sorted().rev().tuples().next() {
                    None => HandType::FiveOfaKind,
                    Some((4, 1)) => HandType::FourOfaKind,
                    Some((3, 2)) => HandType::FullHouse,
                    Some((3, 1)) => HandType::ThreeOfaKind,
                    Some((2, 2)) => HandType::TwoPair,
                    Some((2, 1)) => HandType::OnePair,
                    Some((1, 1)) => HandType::HighCard,
                    _ => unreachable!()
                };
            }
        }

        Hand{
            cards: h.cards.replace("j", "_"),
            bid: h.bid,
            htype
        }
    }).collect();
    game.into_iter().sorted().enumerate().map(|(i, h)| (i+1) * h.bid).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part1(input), 6440);
    }

    #[test]
    fn part2_test() {
        let input = include_str!("../input_test.txt");
        assert_eq!(part2(input), 5905);
    }
}
