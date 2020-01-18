#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;
use crate::error::Error;

/// The weather effect that will be applied at the end of a turn, which causes fires to spread.
#[derive(Debug, Clone)]
pub struct Forecast {
}

impl Forecast {

    /// The direction the wind will blow fires in. Can be 'north', 'east', 'south', or 'west'.
    pub fn direction(&self) -> Str {
        unimplemented!()
    }

    /// How much of a Building's fire that can be blown in the direction of this Forecast. Fire is
    /// duplicated (copied), not moved (transfered).
    pub fn intensity(&self) -> i64 {
        unimplemented!()
    }

    /// The Player that can use WeatherStations to control this Forecast when its the nextForecast.
    pub fn controlling_player(&self) -> Player {
        unimplemented!()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// A unique id for each instance of a GameObject or a sub class. Used for client and server
    /// communication. Should never change value after being set.
    pub fn id(&self) -> Str {
        unimplemented!()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// String representing the top level Class that this game object is an instance of. Used for
    /// reflection to create new instances on clients, but exposed for convenience should AIs want
    /// this data.
    pub fn game_object_name(&self) -> Str {
        unimplemented!()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// Any strings logged will be stored here. Intended for debugging.
    pub fn logs(&self) -> List<Str> {
        unimplemented!()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// Adds a message to this GameObject's logs. Intended for your own debugging purposes, as
    /// strings stored here are saved in the gamelog.
    ///
    /// # Arguments
    ///
    /// - _message_ - A string to add to this GameObject's log. Intended for debugging.
    pub fn log(
        &self,
        _message: &str,
    )
        -> Result<(), Error>
    {
        unimplemented!()
    }
}
