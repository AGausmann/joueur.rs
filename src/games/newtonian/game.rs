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

    /// The number of Tiles in the map along the x (horizontal) axis.
    pub fn map_width(&self) -> i64 {
        unimplemented!()
    }

    /// The number of Tiles in the map along the y (vertical) axis.
    pub fn map_height(&self) -> i64 {
        unimplemented!()
    }

    /// All the tiles in the map, stored in Row-major order. Use `x + y * mapWidth` to access the
    /// correct index.
    pub fn tiles(&self) -> List<Tile> {
        unimplemented!()
    }

    /// Every Unit in the game.
    pub fn units(&self) -> List<Unit> {
        unimplemented!()
    }

    /// A list of all jobs. first item is intern, second is physicists, and third is manager.
    pub fn jobs(&self) -> List<Job> {
        unimplemented!()
    }

    /// Every Machine in the game.
    pub fn machines(&self) -> List<Machine> {
        unimplemented!()
    }

    /// The amount of victory points added when a refined ore is consumed by the generator.
    pub fn refined_value(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of turns it takes a unit to spawn.
    pub fn spawn_time(&self) -> i64 {
        unimplemented!()
    }

    /// The maximum number of managers a player can have.
    pub fn manager_cap(&self) -> i64 {
        unimplemented!()
    }

    /// The maximum number of interns a player can have.
    pub fn intern_cap(&self) -> i64 {
        unimplemented!()
    }

    /// The maximum number of physicists a player can have.
    pub fn physicist_cap(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of turns a unit cannot do anything when stunned.
    pub fn stun_time(&self) -> i64 {
        unimplemented!()
    }

    /// The number turns a unit is immune to being stunned.
    pub fn time_immune(&self) -> i64 {
        unimplemented!()
    }

    /// The number of materials that spawn per spawn cycle.
    pub fn material_spawn(&self) -> i64 {
        unimplemented!()
    }

    /// The percent of max HP regained when a unit end their turn on a tile owned by their player.
    pub fn regenerate_rate(&self) -> f64 {
        unimplemented!()
    }

    /// The amount of combined heat and pressure that you need to win.
    pub fn victory_amount(&self) -> i64 {
        unimplemented!()
    }
}
