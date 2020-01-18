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

    /// Every Unit in the game. Merchant units have targetPort set to a port.
    pub fn units(&self) -> List<Unit> {
        self.inner.lock().unwrap()
            .units.clone()
    }

    /// Every Port in the game. Merchant ports have owner set to None.
    pub fn ports(&self) -> List<Port> {
        self.inner.lock().unwrap()
            .ports.clone()
    }

    /// How much gold it costs to construct a single crew.
    pub fn crew_cost(&self) -> i64 {
        self.inner.lock().unwrap()
            .crew_cost.clone()
    }

    /// How much gold it costs to construct a ship.
    pub fn ship_cost(&self) -> i64 {
        self.inner.lock().unwrap()
            .ship_cost.clone()
    }

    /// How much damage crew deal to each other.
    pub fn crew_damage(&self) -> i64 {
        self.inner.lock().unwrap()
            .crew_damage.clone()
    }

    /// How much damage ships deal to ships and ports.
    pub fn ship_damage(&self) -> i64 {
        self.inner.lock().unwrap()
            .ship_damage.clone()
    }

    /// The maximum amount of health a crew member can have.
    pub fn crew_health(&self) -> i64 {
        self.inner.lock().unwrap()
            .crew_health.clone()
    }

    /// The maximum amount of health a ship can have.
    pub fn ship_health(&self) -> i64 {
        self.inner.lock().unwrap()
            .ship_health.clone()
    }

    /// A crew's attack range. Range is circular.
    pub fn crew_range(&self) -> f64 {
        self.inner.lock().unwrap()
            .crew_range.clone()
    }

    /// A ship's attack range. Range is circular.
    pub fn ship_range(&self) -> f64 {
        self.inner.lock().unwrap()
            .ship_range.clone()
    }

    /// The number of moves Units with only crew are given each turn.
    pub fn crew_moves(&self) -> i64 {
        self.inner.lock().unwrap()
            .crew_moves.clone()
    }

    /// The number of moves Units with ships are given each turn.
    pub fn ship_moves(&self) -> i64 {
        self.inner.lock().unwrap()
            .ship_moves.clone()
    }

    /// How far a Unit can be from a Port to rest. Range is circular.
    pub fn rest_range(&self) -> f64 {
        self.inner.lock().unwrap()
            .rest_range.clone()
    }

    /// How much health a Unit recovers when they rest.
    pub fn heal_factor(&self) -> f64 {
        self.inner.lock().unwrap()
            .heal_factor.clone()
    }

    /// The rate buried gold increases each turn.
    pub fn bury_interest_rate(&self) -> f64 {
        self.inner.lock().unwrap()
            .bury_interest_rate.clone()
    }

    /// When a merchant ship spawns, the amount of additional gold it has relative to the Port's
    /// investment.
    pub fn merchant_interest_rate(&self) -> f64 {
        self.inner.lock().unwrap()
            .merchant_interest_rate.clone()
    }

    /// The Euclidean distance buried gold must be from the Player's Port to accumulate interest.
    pub fn min_interest_distance(&self) -> f64 {
        self.inner.lock().unwrap()
            .min_interest_distance.clone()
    }

    /// How much gold merchant Ports get each turn.
    pub fn merchant_gold_rate(&self) -> f64 {
        self.inner.lock().unwrap()
            .merchant_gold_rate.clone()
    }
}
