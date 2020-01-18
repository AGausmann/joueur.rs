#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;
use crate::error::Error;

/// A machine in the game. Used to refine ore.
#[derive(Debug, Clone)]
pub struct Machine {
}

impl Machine {

    /// The Tile this Machine is on.
    pub fn tile(&self) -> Tile {
        unimplemented!()
    }

    /// Tracks how many times this machine has been worked. (0 to refineTime).
    pub fn worked(&self) -> i64 {
        unimplemented!()
    }

    /// What type of ore the machine takes it. Also determines the type of material it outputs.
    /// (redium or blueium).
    pub fn ore_type(&self) -> Str {
        unimplemented!()
    }

    /// The number of times this machine needs to be worked to refine ore.
    pub fn refine_time(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of ore that needs to be inputted into the machine for it to be worked.
    pub fn refine_input(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of refined ore that is returned after the machine has been fully worked.
    pub fn refine_output(&self) -> i64 {
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
