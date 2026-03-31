use rocket::serde::{json, Serialize, DeserializeOwned};

pub struct Puzzle {
    pub name: String,
    pub evaluate: Box<dyn Fn(&String) -> Result<String, json::serde_json::Error> + Send + Sync>
}

pub fn make_puzzle<T: DeserializeOwned, U: Serialize, H: Fn(&T) -> U + 'static + Send + Sync>(name: String, evaluate: H) -> Puzzle {
    return Puzzle { name, evaluate: Box::new(move |s| -> Result<String, json::serde_json::Error> { json::to_string(&evaluate(&json::from_str(s)?)) }) };
}
