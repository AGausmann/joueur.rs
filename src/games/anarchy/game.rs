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

    /// The width of the entire map along the horizontal (x) axis.
    pub fn map_width(&self) -> i64 {
        unimplemented!()
    }

    /// The width of the entire map along the vertical (y) axis.
    pub fn map_height(&self) -> i64 {
        unimplemented!()
    }

    /// All the buildings in the game.
    pub fn buildings(&self) -> List<Building> {
        unimplemented!()
    }

    /// All the forecasts in the game, indexed by turn number.
    pub fn forecasts(&self) -> List<Forecast> {
        unimplemented!()
    }

    /// The current Forecast, which will be applied at the end of the turn.
    pub fn current_forecast(&self) -> Forecast {
        unimplemented!()
    }

    /// The next Forecast, which will be applied at the end of your opponent's turn. This is also
    /// the Forecast WeatherStations can control this turn.
    pub fn next_forecast(&self) -> Option<Forecast> {
        unimplemented!()
    }

    /// How many bribes players get at the beginning of their turn, not counting their burned down
    /// Buildings.
    pub fn base_bribes_per_turn(&self) -> i64 {
        unimplemented!()
    }

    /// The maximum amount of fire value for any Building.
    pub fn max_fire(&self) -> i64 {
        unimplemented!()
    }

    /// The maximum amount of intensity value for any Forecast.
    pub fn max_forecast_intensity(&self) -> i64 {
        unimplemented!()
    }

    pub fn try_cast<T: Object>(&self) -> Option<T> {
        T::from_game_object(&self.inner, &self.context)
    }

    pub fn cast<T: Object>(&self) -> T {
        self.try_cast().unwrap()
    }
}
