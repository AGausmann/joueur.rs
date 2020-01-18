#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;
use crate::error::Error;

/// Information about a beaver's job.
#[derive(Debug, Clone)]
pub struct Job {
}

impl Job {

    /// The Job title.
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

    /// The number of actions this Job can make per turn.
    pub fn actions(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of damage this Job does per attack.
    pub fn damage(&self) -> i64 {
        unimplemented!()
    }

    /// Scalar for how many branches this Job harvests at once.
    pub fn chopping(&self) -> i64 {
        unimplemented!()
    }

    /// Scalar for how much food this Job harvests at once.
    pub fn munching(&self) -> i64 {
        unimplemented!()
    }

    /// How many turns a beaver attacked by this Job is distracted by.
    pub fn distraction_power(&self) -> i64 {
        unimplemented!()
    }

    /// How many combined resources a beaver with this Job can hold at once.
    pub fn carry_limit(&self) -> i64 {
        unimplemented!()
    }

    /// How much food this Job costs to recruit.
    pub fn cost(&self) -> i64 {
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

    /// Recruits a Beaver of this Job to a lodge
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile that is a lodge owned by you that you wish to spawn the Beaver of this
    /// Job on.
    ///
    /// # Returns
    ///
    /// The recruited Beaver if successful, None otherwise.
    pub fn recruit(
        &self,
        _tile: &Tile,
    )
        -> Result<Option<Beaver>, Error>
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
