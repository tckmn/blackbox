use std::sync::LazyLock;

use crate::puzzle::{Puzzle, make_puzzle};
use crate::puzzle::PuzType::*;

// via @tobia
// https://users.rust-lang.org/t/logarithmic-counting-macro/8918
macro_rules! count {
    () => {0usize};
    ($one:expr) => {1usize};
    ($($pairs:expr, $_p:expr),*) => { count!($($pairs),*) << 1usize };
    ($odd:expr, $($rest:expr),*) => { count!($($rest),*) | 1usize };
}

macro_rules! puzzles {
    ($($x:expr),*) => {
        pub static ALL_PUZZLES: LazyLock<[Puzzle; count!($($x),*)]> = LazyLock::new(|| { [$($x),*] });
    };
}


puzzles!(
    make_puzzle("one".into(), STR, STR, |s: &String| { format!("[{}]", s) }),
    make_puzzle("two".into(), NUM, NUM, |n: &i64| { n + 1 })
);
