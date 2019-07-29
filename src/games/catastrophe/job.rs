#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;

/// Information about a Unit's job.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Job {
}

impl Job {

    /// The Job title.
    pub fn title(&self) -> Str {
        unimplemented!()
    }

    /// The number of moves this Job can make per turn.
    pub fn moves(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of energy this Job normally uses to perform its actions.
    pub fn action_cost(&self) -> f64 {
        unimplemented!()
    }

    /// The amount of energy normally regenerated when resting at a shelter.
    pub fn regen_rate(&self) -> f64 {
        unimplemented!()
    }

    /// How many combined resources a Unit with this Job can hold at once.
    pub fn carry_limit(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of food per turn this Unit consumes. If there isn't enough food for every Unit,
    /// all Units become starved and do not consume food.
    pub fn upkeep(&self) -> i64 {
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
