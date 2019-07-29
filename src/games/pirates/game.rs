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

    /// Every Unit in the game. Merchant units have targetPort set to a port.
    pub fn units(&self) -> List<Unit> {
        unimplemented!()
    }

    /// Every Port in the game. Merchant ports have owner set to None.
    pub fn ports(&self) -> List<Port> {
        unimplemented!()
    }

    /// How much gold it costs to construct a single crew.
    pub fn crew_cost(&self) -> i64 {
        unimplemented!()
    }

    /// How much gold it costs to construct a ship.
    pub fn ship_cost(&self) -> i64 {
        unimplemented!()
    }

    /// How much damage crew deal to each other.
    pub fn crew_damage(&self) -> i64 {
        unimplemented!()
    }

    /// How much damage ships deal to ships and ports.
    pub fn ship_damage(&self) -> i64 {
        unimplemented!()
    }

    /// The maximum amount of health a crew member can have.
    pub fn crew_health(&self) -> i64 {
        unimplemented!()
    }

    /// The maximum amount of health a ship can have.
    pub fn ship_health(&self) -> i64 {
        unimplemented!()
    }

    /// A crew's attack range. Range is circular.
    pub fn crew_range(&self) -> f64 {
        unimplemented!()
    }

    /// A ship's attack range. Range is circular.
    pub fn ship_range(&self) -> f64 {
        unimplemented!()
    }

    /// The number of moves Units with only crew are given each turn.
    pub fn crew_moves(&self) -> i64 {
        unimplemented!()
    }

    /// The number of moves Units with ships are given each turn.
    pub fn ship_moves(&self) -> i64 {
        unimplemented!()
    }

    /// How far a Unit can be from a Port to rest. Range is circular.
    pub fn rest_range(&self) -> f64 {
        unimplemented!()
    }

    /// How much health a Unit recovers when they rest.
    pub fn heal_factor(&self) -> f64 {
        unimplemented!()
    }

    /// The rate buried gold increases each turn.
    pub fn bury_interest_rate(&self) -> f64 {
        unimplemented!()
    }

    /// When a merchant ship spawns, the amount of additional gold it has relative to the Port's
    /// investment.
    pub fn merchant_interest_rate(&self) -> f64 {
        unimplemented!()
    }

    /// The Euclidean distance buried gold must be from the Player's Port to accumulate interest.
    pub fn min_interest_distance(&self) -> f64 {
        unimplemented!()
    }

    /// How much gold merchant Ports get each turn.
    pub fn merchant_gold_rate(&self) -> f64 {
        unimplemented!()
    }
}
