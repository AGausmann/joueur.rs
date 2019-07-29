#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;

/// A location (node) connected to other Nests via Webs (edges) in the game that Spiders can
/// converge on, regardless of owner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Nest {
}

impl Nest {

    /// Webs that connect to this Nest.
    pub fn webs(&self) -> List<Web> {
        unimplemented!()
    }

    /// All the Spiders currently located on this Nest.
    pub fn spiders(&self) -> List<Spider> {
        unimplemented!()
    }

    /// The Player that 'controls' this Nest as they have the most Spiders on this nest.
    pub fn controlling_player(&self) -> Option<Player> {
        unimplemented!()
    }

    /// The X coordinate of the Nest. Used for distance calculations.
    pub fn x(&self) -> i64 {
        unimplemented!()
    }

    /// The Y coordinate of the Nest. Used for distance calculations.
    pub fn y(&self) -> i64 {
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
