#[macro_use] extern crate rocket;

use rocket::State;
use rocket::fs::FileServer;
use rocket::tokio::sync::Mutex;
use rocket::tokio::sync::broadcast;
use rocket::futures::{StreamExt, SinkExt};
use rocket::serde::{Serialize, Deserialize};
use std::sync::Arc;

struct BlackBox {
    channel: broadcast::Sender<BroadcastMessage>,
    guesses: Arc<Mutex<Vec<(String, String)>>>
}

#[derive(Clone)]
enum BroadcastMessage {
    NewGuess(String, String)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
enum InputMessage {
    MakeGuess(String)
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
#[serde(tag = "t", content = "c")]
enum OutputMessage {
    AllGuesses(Vec<(String, String)>),
    OneGuess(String, String)
}

#[get("/ws")]
async fn route_ws<'a>(ws: ws::WebSocket, bb: &'a State<BlackBox>) -> ws::Channel<'a> {
    ws.channel(move |mut stream| Box::pin(async move {
        let mut rx = bb.channel.subscribe();

        loop {
            rocket::tokio::select! {
                msg = rx.recv() => {
                    match msg {
                        Ok(BroadcastMessage::NewGuess(guess, response)) => {
                            let _ = stream.send(rocket::serde::json::to_string(&OutputMessage::OneGuess(guess, response)).unwrap().into()).await;
                        }
                        Err(_) => {}
                    }
                }
                msg = stream.next() => {
                    let Some(msg) = msg else { break };
                    match msg {
                        Ok(ws::Message::Text(txt)) => {
                            match rocket::serde::json::from_str(&txt) {
                                Ok(InputMessage::MakeGuess(guess)) => {
                                    let response = guess.clone();
                                    let _ = bb.channel.send(BroadcastMessage::NewGuess(guess.clone(), response.clone()));
                                    let mut guesses = bb.guesses.lock().await;
                                    guesses.push((guess, response));
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

#[launch]
fn rocket() -> _ {
    let (tx, _rx) = broadcast::channel(99);
    rocket::build()
        .manage(BlackBox {
            channel: tx,
            guesses: Arc::new(Mutex::new(Vec::new()))
        })
        .mount("/", FileServer::from("dist"))
        .mount("/", routes![route_ws])
}
