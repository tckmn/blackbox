#[macro_use] extern crate rocket;

use rocket::State;
use rocket::fs::FileServer;
use rocket::tokio::sync::Mutex;
use rocket::tokio::sync::broadcast;
use rocket::futures::{StreamExt, SinkExt};
use rocket::serde::{json, Serialize, Deserialize};
use std::sync::Arc;
use std::sync::atomic::{Ordering, AtomicUsize};

mod puzzle;
mod all_puzzles;
use all_puzzles::ALL_PUZZLES;

struct BlackBox {
    channel: broadcast::Sender<BroadcastMessage>,
    guesses: Arc<Mutex<Vec<(String, String)>>>,
    puzzle_idx: AtomicUsize
}

impl BlackBox {
    fn puzzle(&self) -> &puzzle::Puzzle { &ALL_PUZZLES[self.puzzle_idx.load(Ordering::Acquire)] }
    fn set_puzzle(&self, idx: usize) { self.puzzle_idx.store(idx, Ordering::Release); }
}

#[derive(Clone)]
enum BroadcastMessage {
    NewGuess(String, String),
    NewPuzzle()
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
enum InputMessage {
    MakeGuess(String)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(tag = "t", content = "c")]
enum OutputMessage<'a> {
    SetPuzzle(&'a String, &'a puzzle::PuzType, &'a puzzle::PuzType),
    AllGuesses(&'a Vec<(String, String)>),
    OneGuess(&'a String, &'a String)
}

macro_rules! send {
    ( $stream:expr, $x:expr ) => {
        let _ = $stream.send(json::to_string(&$x).unwrap().into()).await;
    };
}

macro_rules! broadcast {
    ( $bb:expr, $x:expr ) => {
        let _ = $bb.channel.send($x);
    };
}

#[get("/ws")]
async fn route_ws<'a>(ws: ws::WebSocket, bb: &'a State<BlackBox>) -> ws::Channel<'a> {
    ws.channel(move |mut stream| Box::pin(async move {
        {
            let guesses = bb.guesses.lock().await;
            send!(stream, OutputMessage::SetPuzzle(&bb.puzzle().name, &bb.puzzle().itype, &bb.puzzle().otype));
            send!(stream, OutputMessage::AllGuesses(&*guesses));
        }

        let mut rx = bb.channel.subscribe();
        loop {
            rocket::tokio::select! {
                msg = rx.recv() => {
                    match msg {
                        Ok(BroadcastMessage::NewGuess(guess, response)) => {
                            send!(stream, OutputMessage::OneGuess(&guess, &response));
                        }
                        Ok(BroadcastMessage::NewPuzzle()) => {
                            send!(stream, OutputMessage::SetPuzzle(&bb.puzzle().name, &bb.puzzle().itype, &bb.puzzle().otype));
                        }
                        Err(_) => {}
                    }
                }
                msg = stream.next() => {
                    let Some(msg) = msg else { break };
                    match msg {
                        Ok(ws::Message::Text(txt)) => {
                            match json::from_str(&txt) {
                                Ok(InputMessage::MakeGuess(guess)) => {
                                    if let Ok(response) = (bb.puzzle().evaluate)(&guess) {
                                        broadcast!(bb, BroadcastMessage::NewGuess(guess.clone(), response.clone()));
                                        let mut guesses = bb.guesses.lock().await;
                                        guesses.push((guess, response));
                                    }
                                }
                                Err(e) => { eprintln!("{}", e); }
                            }
                        }
                        Ok(ws::Message::Close(_)) | Err(_) => break,
                        _ => {}
                    }
                }
            }
        }

        Ok(())
    }))
}

#[get("/set/<idx>")]
async fn route_set_puzzle(idx: usize, bb: &State<BlackBox>) -> &'static str {
    let mut guesses = bb.guesses.lock().await;
    guesses.clear();
    bb.set_puzzle(idx);
    broadcast!(bb, BroadcastMessage::NewPuzzle());
    "done"
}

#[launch]
fn rocket() -> _ {
    let (tx, _rx) = broadcast::channel(99);
    rocket::build()
        .manage(BlackBox {
            channel: tx,
            guesses: Arc::new(Mutex::new(Vec::new())),
            puzzle_idx: AtomicUsize::new(0)
        })
        .mount("/", FileServer::from("dist"))
        .mount("/", routes![route_ws, route_set_puzzle])
}
