#[macro_use] extern crate rocket;

use rocket::State;
use rocket::fs::FileServer;
use rocket::tokio::sync::Mutex;
use std::sync::Arc;

struct BlackBox {
    guesses: Arc<Mutex<Vec<String>>>
}

#[get("/<num>")]
async fn f1(num: usize, bb: &State<BlackBox>) -> String {
    let guesses = bb.guesses.lock().await;
    guesses[num].clone()
}

#[get("/<s>", rank=2)]
async fn f2(s: String, bb: &State<BlackBox>) -> String {
    let mut guesses = bb.guesses.lock().await;
    guesses.push(s.clone());
    s.clone()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(BlackBox {
            guesses: Arc::new(Mutex::new(Vec::new()))
        })
        .mount("/", FileServer::from("dist"))
        .mount("/", routes![f1, f2])
}
