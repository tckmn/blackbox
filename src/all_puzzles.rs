use std::sync::LazyLock;

use crate::puzzle::{Puzzle, make_puzzle};

fn id(s: &String) -> String { s.clone() }
fn dumb_puzzle(s: &String) -> String { format!("[{}]", s) }

pub static ALL_PUZZLES: LazyLock<[Puzzle; 1]> = LazyLock::new(|| { [
    make_puzzle(id, id, dumb_puzzle)
] });
