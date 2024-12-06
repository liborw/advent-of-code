pub mod sparse_map;
pub mod map;
pub mod grid;
pub mod position;


pub use took::took;

#[macro_export]
macro_rules! aoc_task {
    ($f:expr) => {
        let (took, result) = took($f);
        println!("{} took: {} result: {}", stringify!($f), took, result);
    };
}
