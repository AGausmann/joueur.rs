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

    /// Every Nest in the game.
    pub fn nests(&self) -> List<Nest> {
        self.inner.lock().unwrap()
            .nests.clone()
    }

    /// Every Web in the game.
    pub fn webs(&self) -> List<Web> {
        self.inner.lock().unwrap()
            .webs.clone()
    }

    /// The speed at which Spiderlings move on Webs.
    pub fn movement_speed(&self) -> i64 {
        self.inner.lock().unwrap()
            .movement_speed.clone()
    }

    /// The speed at which Weavers work to do strengthens and weakens on Webs.
    pub fn weave_speed(&self) -> i64 {
        self.inner.lock().unwrap()
            .weave_speed.clone()
    }

    /// The speed at which Cutters work to do cut Webs.
    pub fn cut_speed(&self) -> i64 {
        self.inner.lock().unwrap()
            .cut_speed.clone()
    }

    /// The speed at which Spitters work to spit new Webs.
    pub fn spit_speed(&self) -> i64 {
        self.inner.lock().unwrap()
            .spit_speed.clone()
    }

    /// How much web strength is added or removed from Webs when they are weaved.
    pub fn weave_power(&self) -> i64 {
        self.inner.lock().unwrap()
            .weave_power.clone()
    }

    /// The starting strength for Webs.
    pub fn initial_web_strength(&self) -> i64 {
        self.inner.lock().unwrap()
            .initial_web_strength.clone()
    }

    /// The maximum strength a web can be strengthened to.
    pub fn max_web_strength(&self) -> i64 {
        self.inner.lock().unwrap()
            .max_web_strength.clone()
    }

    /// Constant used to calculate how many eggs BroodMothers get on their owner's turns.
    pub fn eggs_scalar(&self) -> f64 {
        self.inner.lock().unwrap()
            .eggs_scalar.clone()
    }
}
