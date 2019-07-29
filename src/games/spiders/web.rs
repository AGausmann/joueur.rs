#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;

/// A connection (edge) to a Nest (node) in the game that Spiders can converge on (regardless of
/// owner). Spiders can travel in either direction on Webs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Web {
}

impl Web {

    /// The first Nest this Web is connected to.
    pub fn nest_a(&self) -> Option<Nest> {
        unimplemented!()
    }

    /// The second Nest this Web is connected to.
    pub fn nest_b(&self) -> Option<Nest> {
        unimplemented!()
    }

    /// All the Spiderlings currently moving along this Web.
    pub fn spiderlings(&self) -> List<Spiderling> {
        unimplemented!()
    }

    /// How much weight this Web can take before snapping and destroying itself and all the Spiders
    /// on it.
    pub fn strength(&self) -> i64 {
        unimplemented!()
    }

    /// How much weight this Web currently has on it, which is the sum of all its Spiderlings
    /// weight.
    pub fn load(&self) -> i64 {
        unimplemented!()
    }

    /// How long this Web is, i.e., the distance between its nestA and nestB.
    pub fn length(&self) -> f64 {
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
