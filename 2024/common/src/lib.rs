
pub use took::took;


pub mod map;
pub mod direction;

#[macro_export]
macro_rules! aoc_task {
    ($f:expr) => {
        let (took, result) = took($f);
        println!("{} took: {} result: {}", stringify!($f), took, result);
    };
}

