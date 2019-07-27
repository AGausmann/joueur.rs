#![allow(dead_code, unused_imports)]

use super::*;
use crate::util::*;

/// Gather branches and build up your lodge as beavers fight to survive.
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

    /// Every Beaver in the game.
    pub fn beavers(&self) -> List<Beaver> {
        unimplemented!()
    }

    /// Every Spawner in the game.
    pub fn spawner(&self) -> List<Spawner> {
        unimplemented!()
    }

    /// All the types of Spawners in the game.
    pub fn spawner_types(&self) -> List<String> {
        unimplemented!()
    }

    /// All the Jobs that Beavers can have in the game.
    pub fn jobs(&self) -> List<Job> {
        unimplemented!()
    }

    /// When a Player has less Beavers than this number, then recruiting other Beavers is free.
    pub fn free_beavers_count(&self) -> isize {
        unimplemented!()
    }

    /// How many lodges must be owned by a Player at once to win the game.
    pub fn lodges_to_win(&self) -> isize {
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
