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
    inner: Arc<Mutex<inner::GameBase>>,
}

impl Game {
    pub(crate) fn new(inner: Arc<Mutex<inner::GameBase>>, context: Weak<Mutex<inner::Context>>) -> Game {
        Game { inner, context }
    }

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
        self.inner.lock().unwrap()
            .game_objects.clone()
    }

    /// List of all the players in the game.
    pub fn players(&self) -> List<Player> {
        self.inner.lock().unwrap()
            .players.clone()
    }

    /// A unique identifier for the game instance that is being played.
    pub fn session(&self) -> Str {
        self.inner.lock().unwrap()
            .session.clone()
    }

    /// The player whose turn it is currently. That player can send commands. Other players cannot.
    pub fn current_player(&self) -> Player {
        self.inner.lock().unwrap()
            .current_player.clone()
    }

    /// The current turn number, starting at 0 for the first player's turn.
    pub fn current_turn(&self) -> i64 {
        self.inner.lock().unwrap()
            .current_turn.clone()
    }

    /// The maximum number of turns before the game will automatically end.
    pub fn max_turns(&self) -> i64 {
        self.inner.lock().unwrap()
            .max_turns.clone()
    }

    /// The amount of time (in nano-seconds) added after each player performs a turn.
    pub fn time_added_per_turn(&self) -> i64 {
        self.inner.lock().unwrap()
            .time_added_per_turn.clone()
    }

    /// The number of Tiles in the map along the x (horizontal) axis.
    pub fn map_width(&self) -> i64 {
        self.inner.lock().unwrap()
            .map_width.clone()
    }

    /// The number of Tiles in the map along the y (vertical) axis.
    pub fn map_height(&self) -> i64 {
        self.inner.lock().unwrap()
            .map_height.clone()
    }

    /// All the tiles in the map, stored in Row-major order. Use `x + y * mapWidth` to access the
    /// correct index.
    pub fn tiles(&self) -> List<Tile> {
        self.inner.lock().unwrap()
            .tiles.clone()
    }

    /// Every Unit in the game.
    pub fn units(&self) -> List<Unit> {
        self.inner.lock().unwrap()
            .units.clone()
    }

    /// A list of every unit type / job.
    pub fn unit_jobs(&self) -> List<UnitJob> {
        self.inner.lock().unwrap()
            .unit_jobs.clone()
    }

    /// Every Tower in the game.
    pub fn towers(&self) -> List<Tower> {
        self.inner.lock().unwrap()
            .towers.clone()
    }

    /// A list of every tower type / job.
    pub fn tower_jobs(&self) -> List<TowerJob> {
        self.inner.lock().unwrap()
            .tower_jobs.clone()
    }

    /// The amount of turns it takes between the river changing phases.
    pub fn river_phase(&self) -> i64 {
        self.inner.lock().unwrap()
            .river_phase.clone()
    }

    /// The amount of gold income per turn per unit in a mine.
    pub fn gold_income_per_unit(&self) -> i64 {
        self.inner.lock().unwrap()
            .gold_income_per_unit.clone()
    }

    /// The amount of gold income per turn per unit in the island mine.
    pub fn island_income_per_unit(&self) -> i64 {
        self.inner.lock().unwrap()
            .island_income_per_unit.clone()
    }

    /// The Amount of gold income per turn per unit fishing on the river side.
    pub fn mana_income_per_unit(&self) -> i64 {
        self.inner.lock().unwrap()
            .mana_income_per_unit.clone()
    }
}
