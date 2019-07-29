#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;

/// Holds top-level game state and settings for the current game.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Game {
}

impl Game {

    /// A mapping of every game object's ID to the actual game object. Primarily used by the server
    /// and client to easily refer to the game objects via ID.
    pub fn game_objects(&self) -> Map<Str, GameObject> {
        unimplemented!()
    }

    /// List of all the players in the game.
    pub fn players(&self) -> List<Player> {
        unimplemented!()
    }

    /// A unique identifier for the game instance that is being played.
    pub fn session(&self) -> Str {
        unimplemented!()
    }

    /// The list of [known] moves that have occured in the game, in Standard Algebriac Notation
    /// (SAN) format. The first element is the first move, with the last being the most recent.
    pub fn history(&self) -> List<Str> {
        unimplemented!()
    }

    /// Forsyth-Edwards Notation (fen), a notation that describes the game board state.
    pub fn fen(&self) -> Str {
        unimplemented!()
    }
}
