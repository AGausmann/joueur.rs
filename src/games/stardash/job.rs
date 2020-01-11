#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;

/// Information about a unit's job.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Job {
}

impl Job {

    /// The Job title. 'corvette', 'missileboat', 'martyr', 'transport', or 'miner'. (in this order
    /// from 0-4).
    pub fn title(&self) -> Str {
        unimplemented!()
    }

    /// The amount of starting health this Job has.
    pub fn energy(&self) -> i64 {
        unimplemented!()
    }

    /// The reserve the martyr use to protect allies.
    pub fn shield(&self) -> i64 {
        unimplemented!()
    }

    /// The distance this job can move per turn.
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

    /// How much money it costs to spawn a unit.
    pub fn unit_cost(&self) -> i64 {
        unimplemented!()
    }

    /// The distance at which this job can effect things.
    pub fn range(&self) -> i64 {
        unimplemented!()
    }

    /// _Inherited from GameObject_
    ///
    /// A unique id for each instance of a GameObject or a sub class. Used for client and server
    /// communication. Should never change value after being set.
    pub fn id(&self) -> Str {
        unimplemented!()
    }

    /// _Inherited from GameObject_
    ///
    /// String representing the top level Class that this game object is an instance of. Used for
    /// reflection to create new instances on clients, but exposed for convenience should AIs want
    /// this data.
    pub fn game_object_name(&self) -> Str {
        unimplemented!()
    }

    /// _Inherited from GameObject_
    ///
    /// Any strings logged will be stored here. Intended for debugging.
    pub fn logs(&self) -> List<Str> {
        unimplemented!()
    }

    /// _Inherited from GameObject_
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
    {
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