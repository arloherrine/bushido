mod cards;
mod house;
mod player;
mod game;
mod gamestore;

extern crate iron;
extern crate mount;
extern crate staticfile;
extern crate rand;

use iron::prelude::*;
use iron::status;
use mount::Mount;
use staticfile::Static;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::ops::DerefMut;

use gamestore::{GameStore, MemoryStore};
use game::{Game};

fn main() {
    let mut mount = Mount::new();

    mount.mount("/", Static::new(Path::new("client/index.html")));
    mount.mount("/js/", Static::new(Path::new("client/js")));
    mount.mount("/assets/", Static::new(Path::new("client/assets")));

    let gameStore = Arc::new(Mutex::new(MemoryStore::new()));

    mount.mount("/api/teststart/", move |r: &mut Request| test_start(r, gameStore.lock().unwrap().deref_mut()));

    println!("Server running on http://localhost:3000/");

    Iron::new(mount).http("localhost:3000").unwrap();

}

fn test_start(r: &mut Request, gameStore: &mut MemoryStore) -> IronResult<Response> {
    let game = match gameStore.get(r.url.path()[0]) {
        Some(game) => game,
        None => Game::new(&["Greg", "Bob", "Sally", "Jeff"])
    };
    let serialized = game.serialize(0);
    let moves = game.getMoves();

    let resp = format!("{{\"state\":\n{},\n\n\"moves\":\n{}\n}}", serialized, moves);

    Ok(Response::with((status::Ok, resp)))
}