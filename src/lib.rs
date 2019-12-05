extern crate aoc_runner;

#[macro_use]
extern crate aoc_runner_derive;

// #[macro_use]
// extern crate itertools;

// #[macro_use]
// extern crate lazy_static;

macro_rules! debug_print {
    ($fmt:expr) => {
        #[cfg(feature = "debugging")]
        {
            println!($fmt);
        }
    };
    ($fmt:expr, $($arg:tt)*) => {
        #[cfg(feature = "debugging")]
        {
            println!($fmt, $($arg)*);
        }
    };
}

pub mod day1;
pub mod day2;
pub mod day3;
pub mod day4;
pub mod day5;

aoc_lib! { year = 2019 }
