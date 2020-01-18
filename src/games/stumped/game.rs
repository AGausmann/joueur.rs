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

    /// Every Beaver in the game.
    pub fn beavers(&self) -> List<Beaver> {
        self.inner.lock().unwrap()
            .beavers.clone()
    }

    /// Every Spawner in the game.
    pub fn spawner(&self) -> List<Spawner> {
        self.inner.lock().unwrap()
            .spawner.clone()
    }

    /// All the types of Spawners in the game.
    pub fn spawner_types(&self) -> List<Str> {
        self.inner.lock().unwrap()
            .spawner_types.clone()
    }

    /// All the Jobs that Beavers can have in the game.
    pub fn jobs(&self) -> List<Job> {
        self.inner.lock().unwrap()
            .jobs.clone()
    }

    /// When a Player has less Beavers than this number, then recruiting other Beavers is free.
    pub fn free_beavers_count(&self) -> i64 {
        self.inner.lock().unwrap()
            .free_beavers_count.clone()
    }

    /// How many lodges must be owned by a Player at once to win the game.
    pub fn lodges_to_win(&self) -> i64 {
        self.inner.lock().unwrap()
            .lodges_to_win.clone()
    }

    /// Constant number used to calculate what it costs to spawn a new lodge.
    pub fn lodge_cost_constant(&self) -> f64 {
        self.inner.lock().unwrap()
            .lodge_cost_constant.clone()
    }

    /// Constant number used to calculate how many branches/food Beavers harvest from Spawners.
    pub fn spawner_harvest_constant(&self) -> f64 {
        self.inner.lock().unwrap()
            .spawner_harvest_constant.clone()
    }
}
