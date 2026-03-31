pub struct Puzzle {
    pub evaluate: Box<dyn Fn(&String) -> String + Send + Sync>
}

pub fn make_puzzle<T, U, F: Fn(&String) -> T + 'static + Send + Sync, G: Fn(&U) -> String + 'static + Send + Sync, H: Fn(&T) -> U + 'static + Send + Sync>(from_guess: F, to_response: G, evaluate: H) -> Puzzle {
    return Puzzle { evaluate: Box::new(move |s| -> String { to_response(&evaluate(&from_guess(s))) }) };
}
