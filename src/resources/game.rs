use crate::components::MovementOptions;
use core::fmt;

pub struct Game {
    pub phase: Phase,
}

impl Game {
    pub fn new() -> Game {
        Game{
            phase: Phase::AITurn,
        }
    }
}

impl Default for Game {
    fn default() -> Game {
        Game::new()
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum Phase {
    AITurn,
    PlayerTurn,
    PlayerAction,
    AIAction,
}

impl fmt::Display for Phase {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}