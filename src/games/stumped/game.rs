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

    /// Every Beaver in the game.
    pub fn beavers(&self) -> List<Beaver> {
        unimplemented!()
    }

    /// Every Spawner in the game.
    pub fn spawner(&self) -> List<Spawner> {
        unimplemented!()
    }

    /// All the types of Spawners in the game.
    pub fn spawner_types(&self) -> List<Str> {
        unimplemented!()
    }

    /// All the Jobs that Beavers can have in the game.
    pub fn jobs(&self) -> List<Job> {
        unimplemented!()
    }

    /// When a Player has less Beavers than this number, then recruiting other Beavers is free.
    pub fn free_beavers_count(&self) -> i64 {
        unimplemented!()
    }

    /// How many lodges must be owned by a Player at once to win the game.
    pub fn lodges_to_win(&self) -> i64 {
        unimplemented!()
    }

    /// Constant number used to calculate what it costs to spawn a new lodge.
    pub fn lodge_cost_constant(&self) -> f64 {
        unimplemented!()
    }

    /// Constant number used to calculate how many branches/food Beavers harvest from Spawners.
    pub fn spawner_harvest_constant(&self) -> f64 {
        unimplemented!()
    }
}
