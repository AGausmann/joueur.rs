#![allow(dead_code, unused_imports)]

use super::*;
use crate::util::*;

/// Use cowboys to have a good time and play some music on a Piano, while brawling with enemy
/// Cowboys.
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

    /// The number of Tiles in the map along the x (horizontal) axis.
    pub fn map_width(&self) -> isize {
        unimplemented!()
    }

    /// The number of Tiles in the map along the y (vertical) axis.
    pub fn map_height(&self) -> isize {
        unimplemented!()
    }

    /// All the tiles in the map, stored in Row-major order. Use `x + y * mapWidth` to access the
    /// correct index.
    pub fn tiles(&self) -> List<Tile> {
        unimplemented!()
    }

    /// Every Cowboy in the game.
    pub fn cowboys(&self) -> List<Cowboy> {
        unimplemented!()
    }

    /// Every furnishing in the game.
    pub fn furnishings(&self) -> List<Furnishing> {
        unimplemented!()
    }

    /// All the jobs that Cowboys can be called in with.
    pub fn jobs(&self) -> List<String> {
        unimplemented!()
    }

    /// All the beer Bottles currently flying across the saloon in the game.
    pub fn bottles(&self) -> List<Bottle> {
        unimplemented!()
    }

    /// When a player's rowdiness reaches or exceeds this number their Cowboys take a collective
    /// siesta.
    pub fn rowdiness_to_siesta(&self) -> isize {
        unimplemented!()
    }

    /// How long siestas are for a player's team.
    pub fn siesta_length(&self) -> isize {
        unimplemented!()
    }

    /// The maximum number of Cowboys a Player can bring into the saloon of each specific job.
    pub fn max_cowboys_per_job(&self) -> isize {
        unimplemented!()
    }

    /// How much damage is applied to things hit by Sharpshooters when they act.
    pub fn sharpshooter_damage(&self) -> isize {
        unimplemented!()
    }

    /// How much damage is applied to neighboring things bit by the Sharpshooter between turns.
    pub fn brawler_damage(&self) -> isize {
        unimplemented!()
    }

    /// How many turns a Cowboy will be drunk for if a bottle breaks on it.
    pub fn turns_drunk(&self) -> isize {
        unimplemented!()
    }

    /// How many turns a Bartender will be busy for after throwing a Bottle.
    pub fn bartender_cooldown(&self) -> isize {
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
