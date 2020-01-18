#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;
use crate::error::Error;

/// A celestial body located within the game.
#[derive(Debug, Clone)]
pub struct Body {
}

impl Body {

    /// The Player that owns and can control this Body.
    pub fn owner(&self) -> Option<Player> {
        unimplemented!()
    }

    /// The x value this celestial body is on.
    pub fn x(&self) -> f64 {
        unimplemented!()
    }

    /// The y value this celestial body is on.
    pub fn y(&self) -> f64 {
        unimplemented!()
    }

    /// The radius of the circle that this body takes up.
    pub fn radius(&self) -> f64 {
        unimplemented!()
    }

    /// The type of celestial body it is. Either 'planet', 'asteroid', or 'sun'.
    pub fn body_type(&self) -> Str {
        unimplemented!()
    }

    /// The type of material the celestial body has. Either 'none', 'genarium', 'rarium',
    /// 'legendarium', or 'mythicite'.
    pub fn material_type(&self) -> Str {
        unimplemented!()
    }

    /// The amount of material the object has, or energy if it is a planet.
    pub fn amount(&self) -> i64 {
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

    /// Spawn a unit on some value of this celestial body.
    ///
    /// # Arguments
    ///
    /// - _x_ - The x value of the spawned unit.
    ///
    /// - _y_ - The y value of the spawned unit.
    ///
    /// - _title_ - The job title of the unit being spawned.
    ///
    /// # Returns
    ///
    /// True if successfully taken, false otherwise.
    pub fn spawn(
        &self,
        _x: f64,
        _y: f64,
        _title: &str,
    )
        -> Result<bool, Error>
    {
        unimplemented!()
    }

    /// The x value of this body a number of turns from now. (0-how many you want).
    ///
    /// # Arguments
    ///
    /// - _num_ - The number of turns in the future you wish to check.
    ///
    /// # Returns
    ///
    /// The x position of the body the input number of turns in the future.
    pub fn next_x(
        &self,
        _num: i64,
    )
        -> Result<i64, Error>
    {
        unimplemented!()
    }

    /// The x value of this body a number of turns from now. (0-how many you want).
    ///
    /// # Arguments
    ///
    /// - _num_ - The number of turns in the future you wish to check.
    ///
    /// # Returns
    ///
    /// The x position of the body the input number of turns in the future.
    pub fn next_y(
        &self,
        _num: i64,
    )
        -> Result<i64, Error>
    {
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
