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

    /// The player whose turn it is currently. That player can send commands. Other players cannot.
    pub fn current_player(&self) -> Player {
        unimplemented!()
    }

    /// The current turn number, starting at 0 for the first player's turn.
    pub fn current_turn(&self) -> i64 {
        unimplemented!()
    }

    /// The maximum number of turns before the game will automatically end.
    pub fn max_turns(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of time (in nano-seconds) added after each player performs a turn.
    pub fn time_added_per_turn(&self) -> i64 {
        unimplemented!()
    }

    /// The width of the board for X component of a checker.
    pub fn board_width(&self) -> i64 {
        unimplemented!()
    }

    /// The height of the board for the Y component of a checker.
    pub fn board_height(&self) -> i64 {
        unimplemented!()
    }

    /// All the checkers currently in the game.
    pub fn checkers(&self) -> List<Checker> {
        unimplemented!()
    }

    /// The checker that last moved and must be moved because only one checker can move during each
    /// players turn.
    pub fn checker_moved(&self) -> Option<Checker> {
        unimplemented!()
    }

    /// If the last checker that moved jumped, meaning it can move again.
    pub fn checker_moved_jumped(&self) -> bool {
        unimplemented!()
    }
}
