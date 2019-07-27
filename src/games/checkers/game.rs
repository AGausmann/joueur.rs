#![allow(dead_code, unused_imports)]

use super::*;
use crate::util::*;

/// The simple version of American Checkers. An 8x8 board with 12 checkers on each side that must
/// move diagonally to the opposing side until kinged.
pub struct Game {
}

impl Game {

    /// A mapping of every game object's ID to the actual game object. Primarily used by the server
    /// and client to easily refer to the game objects via ID.
    pub fn game_objects(&self) -> Map<String, GameObject> {
        unimplemented!()
    }

    /// List of all the players in the game.
    pub fn players(&self) -> List<Player> {
        unimplemented!()
    }

    /// A unique identifier for the game instance that is being played.
    pub fn session(&self) -> String {
        unimplemented!()
    }

    /// The player whose turn it is currently. That player can send commands. Other players cannot.
    pub fn current_player(&self) -> Player {
        unimplemented!()
    }

    /// The current turn number, starting at 0 for the first player's turn.
    pub fn current_turn(&self) -> isize {
        unimplemented!()
    }

    /// The maximum number of turns before the game will automatically end.
    pub fn max_turns(&self) -> isize {
        unimplemented!()
    }

    /// The amount of time (in nano-seconds) added after each player performs a turn.
    pub fn time_added_per_turn(&self) -> isize {
        unimplemented!()
    }

    /// The width of the board for X component of a checker.
    pub fn board_width(&self) -> isize {
        unimplemented!()
    }

    /// The height of the board for the Y component of a checker.
    pub fn board_height(&self) -> isize {
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

    /// Attempts to cast this object into an object of another class.
    ///
    /// # Errors
    ///
    /// This method will return `None` if this object cannot be casted into the target class. This
    /// happens when the base class of this object does not inherit from the target class.
    pub fn try_cast<T>(&self) -> Option<T> {
        unimplemented!()
    }

    /// Attempts to cast this object into an object of another class.
    ///
    /// # Panics
    ///
    /// Panics if the base class of this object does not inherit from the target class.
    pub fn cast<T>(&self) -> T {
        self.try_cast().unwrap()
    }
}
