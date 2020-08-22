// Player

use std::fmt::{Display, Formatter, Result};

pub struct Player {
    name: String,   // Display Name
    human: bool,    // Is this Human or Bot?
    active: bool,
    score: u32,
}

impl Player {
    pub fn new(name: String) -> Player {
        Player{ name: name, human: true, active: true, score: 0 }
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result { 
        write!(f, "{}", self.name)
    }
}
