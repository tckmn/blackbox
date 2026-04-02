use rocket::serde::{json, Serialize, DeserializeOwned};

pub struct Puzzle {
    pub name: String,
    pub itype: PuzType,
    pub otype: PuzType,
    pub evaluate: Box<dyn Fn(&String) -> Result<String, json::serde_json::Error> + Send + Sync>
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub enum PuzType {
    STR, NUM
}

pub fn make_puzzle<T: DeserializeOwned, U: Serialize, H: Fn(&T) -> U + 'static + Send + Sync>(name: String, itype: PuzType, otype: PuzType, evaluate: H) -> Puzzle {
    return Puzzle {
        name, itype, otype,
        evaluate: Box::new(move |s| { json::to_string(&evaluate(&json::from_str(s)?)) })
    };
}
