#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;
use crate::error::Error;

/// Information about a unit's job.
#[derive(Debug, Clone)]
pub struct Job {
}

impl Job {

    /// The Job title. 'intern', 'manager', or 'physicist'.
    pub fn title(&self) -> Str {
        unimplemented!()
    }

    /// The amount of starting health this Job has.
    pub fn health(&self) -> i64 {
        unimplemented!()
    }

    /// The number of moves this Job can make per turn.
    pub fn moves(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of damage this Job does per attack.
    pub fn damage(&self) -> i64 {
        unimplemented!()
    }

    /// How many combined resources a unit with this Job can hold at once.
    pub fn carry_limit(&self) -> i64 {
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
