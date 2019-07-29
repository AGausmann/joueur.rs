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

    /// Every Structure in the game.
    pub fn structures(&self) -> List<Structure> {
        unimplemented!()
    }

    /// All the Jobs that Units can have in the game.
    pub fn jobs(&self) -> List<Job> {
        unimplemented!()
    }

    /// The multiplier for the amount of energy regenerated when resting in a shelter with the cat
    /// overlord.
    pub fn cat_energy_mult(&self) -> f64 {
        unimplemented!()
    }

    /// The multiplier for the amount of energy regenerated when resting while starving.
    pub fn starving_energy_mult(&self) -> f64 {
        unimplemented!()
    }

    /// The multiplier for the cost of actions when performing them in range of a monument. Does
    /// not effect pickup cost.
    pub fn monument_cost_mult(&self) -> f64 {
        unimplemented!()
    }

    /// The amount of turns it takes for a Tile that was just harvested to grow food again.
    pub fn harvest_cooldown(&self) -> i64 {
        unimplemented!()
    }

    /// The number of turns between fresh humans being spawned on the road.
    pub fn turns_to_create_human(&self) -> i64 {
        unimplemented!()
    }

    /// The number of turns before the harvest rate is lowered (length of each season basically).
    pub fn turns_to_lower_harvest(&self) -> i64 {
        unimplemented!()
    }

    /// The amount that the harvest rate is lowered each season.
    pub fn lower_harvest_amount(&self) -> i64 {
        unimplemented!()
    }

    /// After a food tile is harvested, the number of turns before it can be harvested again.
    pub fn turns_between_harvests(&self) -> i64 {
        unimplemented!()
    }

    /// The number of materials in a neutral Structure.
    pub fn neutral_materials(&self) -> i64 {
        unimplemented!()
    }

    /// The number of materials in a wall.
    pub fn wall_materials(&self) -> i64 {
        unimplemented!()
    }

    /// The number of materials in a shelter.
    pub fn shelter_materials(&self) -> i64 {
        unimplemented!()
    }

    /// The number of materials in a monument.
    pub fn monument_materials(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of food Players start with.
    pub fn starting_food(&self) -> i64 {
        unimplemented!()
    }
}