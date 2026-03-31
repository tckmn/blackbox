use std::sync::LazyLock;

use crate::puzzle::{Puzzle, make_puzzle};

fn dumb_puzzle(s: &String) -> String { format!("[{}]", s) }

pub static ALL_PUZZLES: LazyLock<[Puzzle; 1]> = LazyLock::new(|| { [
    make_puzzle("dumb".into(), dumb_puzzle)
] });
