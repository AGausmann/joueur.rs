#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;
use crate::error::Error;

/// A structure on a Tile.
#[derive(Debug, Clone)]
pub struct Structure {
}

impl Structure {

    /// The type of Structure this is ('shelter', 'monument', 'wall', 'road', 'neutral').
    pub fn type_(&self) -> Str {
        unimplemented!()
    }

    /// The Tile this Structure is on.
    pub fn tile(&self) -> Option<Tile> {
        unimplemented!()
    }

    /// The owner of this Structure if any, otherwise None.
    pub fn owner(&self) -> Option<Player> {
        unimplemented!()
    }

    /// The number of materials in this Structure. Once this number reaches 0, this Structure is
    /// destroyed.
    pub fn materials(&self) -> i64 {
        unimplemented!()
    }

    /// The range of this Structure's effect. For example, a radius of 1 means this Structure
    /// affects a 3x3 square centered on this Structure.
    pub fn effect_radius(&self) -> i64 {
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
