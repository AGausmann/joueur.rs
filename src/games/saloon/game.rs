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

    /// Every Cowboy in the game.
    pub fn cowboys(&self) -> List<Cowboy> {
        self.inner.lock().unwrap()
            .cowboys.clone()
    }

    /// Every furnishing in the game.
    pub fn furnishings(&self) -> List<Furnishing> {
        self.inner.lock().unwrap()
            .furnishings.clone()
    }

    /// All the jobs that Cowboys can be called in with.
    pub fn jobs(&self) -> List<Str> {
        self.inner.lock().unwrap()
            .jobs.clone()
    }

    /// All the beer Bottles currently flying across the saloon in the game.
    pub fn bottles(&self) -> List<Bottle> {
        self.inner.lock().unwrap()
            .bottles.clone()
    }

    /// When a player's rowdiness reaches or exceeds this number their Cowboys take a collective
    /// siesta.
    pub fn rowdiness_to_siesta(&self) -> i64 {
        self.inner.lock().unwrap()
            .rowdiness_to_siesta.clone()
    }

    /// How long siestas are for a player's team.
    pub fn siesta_length(&self) -> i64 {
        self.inner.lock().unwrap()
            .siesta_length.clone()
    }

    /// The maximum number of Cowboys a Player can bring into the saloon of each specific job.
    pub fn max_cowboys_per_job(&self) -> i64 {
        self.inner.lock().unwrap()
            .max_cowboys_per_job.clone()
    }

    /// How much damage is applied to things hit by Sharpshooters when they act.
    pub fn sharpshooter_damage(&self) -> i64 {
        self.inner.lock().unwrap()
            .sharpshooter_damage.clone()
    }

    /// How much damage is applied to neighboring things bit by the Sharpshooter between turns.
    pub fn brawler_damage(&self) -> i64 {
        self.inner.lock().unwrap()
            .brawler_damage.clone()
    }

    /// How many turns a Cowboy will be drunk for if a bottle breaks on it.
    pub fn turns_drunk(&self) -> i64 {
        self.inner.lock().unwrap()
            .turns_drunk.clone()
    }

    /// How many turns a Bartender will be busy for after throwing a Bottle.
    pub fn bartender_cooldown(&self) -> i64 {
        self.inner.lock().unwrap()
            .bartender_cooldown.clone()
    }
}
