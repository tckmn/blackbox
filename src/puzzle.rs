use rocket::serde::{json, Serialize, DeserializeOwned};

pub struct Puzzle {
    pub name: String,
    pub itype: PuzType,
    pub otype: PuzType,
    pub evaluate: Box<dyn Fn(&String) -> Option<String> + Send + Sync>
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
pub enum PuzType {
    STR, NUM
}

pub fn make_puzzle<T: DeserializeOwned, U: Serialize, H: Fn(&T) -> Option<U> + 'static + Send + Sync>(name: String, itype: PuzType, otype: PuzType, evaluate: H) -> Puzzle {
    return Puzzle {
        name, itype, otype,
        evaluate: Box::new(move |s| { evaluate(&json::from_str(s).ok()?).and_then(|x| json::to_string(&x).ok()) })
    };
}
