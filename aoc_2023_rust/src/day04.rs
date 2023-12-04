use std::collections::HashSet;


type Game = (usize, HashSet<u32>, HashSet<u32>);

fn input() -> Vec<Game> {
    include_str!("./input/day04.txt")
        .lines().enumerate().map(|(i, l)| {
            l.split_once(":")
                .and_then(|(_, v)| v.split_once("|"))
                .and_then(|(w, m)| {
                    Some((
                            i,
                            w.split_whitespace().map(|v| v.parse::<u32>().unwrap()).collect(),
                            m.split_whitespace().map(|v| v.parse::<u32>().unwrap()).collect(),
                            ))
                }).unwrap()
        }).collect()
}


pub fn day04a() -> isize {
    input().into_iter().map(|(_, w, m)| {
        w.intersection(&m).fold(0, |acc, _| {
            if acc == 0 { 1 }
            else { acc * 2 }
        } )
    }).sum()
}

pub fn day04b() -> usize {
    let mut deck: Vec<(usize, usize, usize)> = input().into_iter().map(|(i, w, m)| {
        (i, w.intersection(&m).collect::<Vec<_>>().len(), 1)
    }).collect();

    let mut n: usize = 0;

    for i in 0..deck.len() {
        let (i, w, c) = deck[i];
        n += c;
        for j in (i + 1)..(w + i + 1) {
            deck[j].2 += c;
        }
    }
    n
}
