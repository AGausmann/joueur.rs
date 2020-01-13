#![allow(dead_code, unused_imports)]

use std::sync::{Arc, Mutex, Weak};
use std::cell::{RefCell, RefMut};

use super::*;
use crate::types::*;

/// Holds top-level game state and settings for the current game.
#[derive(Debug, Clone)]
pub struct Game {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<GameInner>>,
}

#[derive(Debug, Clone)]
struct GameInner {
    game: Arc<Mutex<GameBase>>,
}

#[derive(Debug)]
pub(crate) struct GameBase {
    pub(crate) game_objects: Map<Str, GameObject>,
    pub(crate) players: List<Player>,
    pub(crate) session: Str,
    pub(crate) current_player: Player,
    pub(crate) current_turn: i64,
    pub(crate) max_turns: i64,
    pub(crate) time_added_per_turn: i64,
    pub(crate) map_width: i64,
    pub(crate) map_height: i64,
    pub(crate) buildings: List<Building>,
    pub(crate) forecasts: List<Forecast>,
    pub(crate) current_forecast: Forecast,
    pub(crate) next_forecast: Option<Forecast>,
    pub(crate) base_bribes_per_turn: i64,
    pub(crate) max_fire: i64,
    pub(crate) max_forecast_intensity: i64,
}

impl Game {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<GameInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Game = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// A mapping of every game object's ID to the actual game object. Primarily used by the server
    /// and client to easily refer to the game objects via ID.
    pub fn game_objects(&self) -> Map<Str, GameObject> {
        self.inner().game.lock().unwrap().game_objects.clone()
    }

    /// List of all the players in the game.
    pub fn players(&self) -> List<Player> {
        self.inner().game.lock().unwrap().players.clone()
    }

    /// A unique identifier for the game instance that is being played.
    pub fn session(&self) -> Str {
        self.inner().game.lock().unwrap().session.clone()
    }

    /// The player whose turn it is currently. That player can send commands. Other players cannot.
    pub fn current_player(&self) -> Player {
        self.inner().game.lock().unwrap().current_player.clone()
    }

    /// The current turn number, starting at 0 for the first player's turn.
    pub fn current_turn(&self) -> i64 {
        self.inner().game.lock().unwrap().current_turn.clone()
    }

    /// The maximum number of turns before the game will automatically end.
    pub fn max_turns(&self) -> i64 {
        self.inner().game.lock().unwrap().max_turns.clone()
    }

    /// The amount of time (in nano-seconds) added after each player performs a turn.
    pub fn time_added_per_turn(&self) -> i64 {
        self.inner().game.lock().unwrap().time_added_per_turn.clone()
    }

    /// The width of the entire map along the horizontal (x) axis.
    pub fn map_width(&self) -> i64 {
        self.inner().game.lock().unwrap().map_width.clone()
    }

    /// The width of the entire map along the vertical (y) axis.
    pub fn map_height(&self) -> i64 {
        self.inner().game.lock().unwrap().map_height.clone()
    }

    /// All the buildings in the game.
    pub fn buildings(&self) -> List<Building> {
        self.inner().game.lock().unwrap().buildings.clone()
    }

    /// All the forecasts in the game, indexed by turn number.
    pub fn forecasts(&self) -> List<Forecast> {
        self.inner().game.lock().unwrap().forecasts.clone()
    }

    /// The current Forecast, which will be applied at the end of the turn.
    pub fn current_forecast(&self) -> Forecast {
        self.inner().game.lock().unwrap().current_forecast.clone()
    }

    /// The next Forecast, which will be applied at the end of your opponent's turn. This is also
    /// the Forecast WeatherStations can control this turn.
    pub fn next_forecast(&self) -> Option<Forecast> {
        self.inner().game.lock().unwrap().next_forecast.clone()
    }

    /// How many bribes players get at the beginning of their turn, not counting their burned down
    /// Buildings.
    pub fn base_bribes_per_turn(&self) -> i64 {
        self.inner().game.lock().unwrap().base_bribes_per_turn.clone()
    }

    /// The maximum amount of fire value for any Building.
    pub fn max_fire(&self) -> i64 {
        self.inner().game.lock().unwrap().max_fire.clone()
    }

    /// The maximum amount of intensity value for any Forecast.
    pub fn max_forecast_intensity(&self) -> i64 {
        self.inner().game.lock().unwrap().max_forecast_intensity.clone()
    }

    pub fn try_cast<T>(&self) -> Option<T> {
        self.context().try_get_obj(&self.id)
    }

    pub fn cast<T>(&self) -> Option<T> {
        self.context().get_obj(&self.id)
    }
}
