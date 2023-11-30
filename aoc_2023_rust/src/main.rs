mod day01;

use std::env;
use took::took;

trait InRange: Sized {
    fn in_range(self, lb: i32, ub: i32) -> Option<Self>;
}

impl InRange for i32 {
    fn in_range(self, lb: i32, ub: i32) -> Option<Self> {
        if (self >= lb) & (self <= ub) {
            Some(self)
        } else {
            None
        }
    }
}

macro_rules! aoc_task {
    ($f:ident) => {
        let (took, result) = took($f);
        println!("{} took: {} result: {}", stringify!($f), took, result);
    };
}

fn main() {

    let day: i32 = env::args().nth(1)
                              .expect("You must provide argument")
                              .parse::<i32>()
                              .expect("The argument must be number")
                              .in_range(1, 24)
                              .expect("The argument should be within 1 and 24");

    match day {
        1 => {
            use day01::*;
            aoc_task!(day01a);
            aoc_task!(day01b);
        }
        _ => ()
    }
}
