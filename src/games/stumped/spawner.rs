#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;
use crate::error::Error;

/// A resource spawner that generates branches or food.
#[derive(Debug, Clone)]
pub struct Spawner {
}

impl Spawner {

    /// What type of resource this is ('food' or 'branches').
    pub fn type_(&self) -> Str {
        unimplemented!()
    }

    /// How much health this Spawner has, which is used to calculate how much of its resource can
    /// be harvested.
    pub fn health(&self) -> i64 {
        unimplemented!()
    }

    /// The Tile this Spawner is on.
    pub fn tile(&self) -> Tile {
        unimplemented!()
    }

    /// True if this Spawner has been harvested this turn, and it will not heal at the end of the
    /// turn, false otherwise.
    pub fn has_been_harvested(&self) -> bool {
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
