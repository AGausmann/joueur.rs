#![allow(dead_code, unused_imports)]

use super::*;
use crate::util::*;

/// Two player grid based game where each player tries to burn down the other player's buildings.
/// Let it burn.
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

    /// The width of the entire map along the horizontal (x) axis.
    pub fn map_width(&self) -> isize {
        unimplemented!()
    }

    /// The width of the entire map along the vertical (y) axis.
    pub fn map_height(&self) -> isize {
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
    pub fn base_bribes_per_turn(&self) -> isize {
        unimplemented!()
    }

    /// The maximum amount of fire value for any Building.
    pub fn max_fire(&self) -> isize {
        unimplemented!()
    }

    /// The maximum amount of intensity value for any Forecast.
    pub fn max_forecast_intensity(&self) -> isize {
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
