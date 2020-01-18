#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// Holds top-level game state and settings for the current game.
#[derive(Debug, Clone)]
pub struct Game {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::GameObject>>,
}

impl Game {
    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }

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

    /// A list of every unit type / job.
    pub fn unit_jobs(&self) -> List<UnitJob> {
        unimplemented!()
    }

    /// Every Tower in the game.
    pub fn towers(&self) -> List<Tower> {
        unimplemented!()
    }

    /// A list of every tower type / job.
    pub fn tower_jobs(&self) -> List<TowerJob> {
        unimplemented!()
    }

    /// The amount of turns it takes between the river changing phases.
    pub fn river_phase(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of gold income per turn per unit in a mine.
    pub fn gold_income_per_unit(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of gold income per turn per unit in the island mine.
    pub fn island_income_per_unit(&self) -> i64 {
        unimplemented!()
    }

    /// The Amount of gold income per turn per unit fishing on the river side.
    pub fn mana_income_per_unit(&self) -> i64 {
        unimplemented!()
    }
}
