use game::{Game};

use std::collections::HashMap;

pub trait GameStore {
    fn get(&mut self, id: &str) -> Option<Game>;
    fn store(&mut self, id: &str, game: Game); // TODO return a Result to indicate failure
}

pub struct MemoryStore {
    dict: HashMap<String, Game>,
}

impl MemoryStore {
    pub fn new() -> MemoryStore {
        MemoryStore { dict: HashMap::new() }
    }
}

impl GameStore for MemoryStore {
    fn get(&mut self, id: &str) -> Option<Game> {
        self.dict.remove(id)
    }

    fn store(&mut self, id: &str, game: Game) { // TODO return a Result to indicate failure
        self.dict.insert(id.to_string(), game);
    }
}