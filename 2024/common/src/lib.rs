
use std::{collections::HashMap, hash::Hash};

pub use took::took;

#[macro_export]
macro_rules! aoc_task {
    ($f:expr) => {
        let (took, result) = took($f);
        println!("{} took: {} result: {}", stringify!($f), took, result);
    };
}



pub fn counts<I, T>(input: I) -> HashMap<T, usize>
where
    I: IntoIterator<Item = T>,
    T: Hash + Eq
{
    let mut m = HashMap::new();
    input.into_iter().for_each(|v| {
        *m.entry(v).or_default() += 1;
    });
    m
}


