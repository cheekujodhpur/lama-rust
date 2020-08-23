// Game

use std::vec::{Vec};

use crate::player::{Player};

const MAX_PLAYERS: usize = 6;

pub struct Game {
    players: Vec<Player>,
}

impl Game {
    pub fn new() -> Game {
        Game{ players: Vec::<Player>::with_capacity(MAX_PLAYERS) }
    }

    pub fn add_player(&mut self, p: Player) {
        if self.players.len() >= MAX_PLAYERS {
            println!("Cannot push more");
        }
        else {
            self.players.push(p);
        }
    }

    pub fn print_players(self) {
        for p in self.players {
            println!("{}", p);
        }
    }
}
