pub mod map;
pub mod grid;
pub mod position;
pub mod vector;
pub mod direction;


pub use took::took;

#[macro_export]
macro_rules! aoc_task {
    ($f:expr) => {
        let (took, result) = took($f);
        println!("{} took: {} result: {}", stringify!($f), took, result);
    };
}
