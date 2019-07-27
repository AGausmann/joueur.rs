#![allow(dead_code, unused_imports)]

use super::*;
use crate::util::*;

/// An furnishing in the Saloon that must be pathed around, or destroyed.
pub struct Furnishing {
}

impl Furnishing {

    /// True if this Furnishing is a piano and can be played, False otherwise.
    pub fn is_piano(&self) -> bool {
        unimplemented!()
    }

    /// The Tile that this Furnishing is located on.
    pub fn tile(&self) -> Option<Tile> {
        unimplemented!()
    }

    /// How much health this Furnishing currently has.
    pub fn health(&self) -> isize {
        unimplemented!()
    }

    /// If this Furnishing has been destroyed, and has been removed from the game.
    pub fn is_destroyed(&self) -> bool {
        unimplemented!()
    }

    /// If this is a piano and a Cowboy is playing it this turn.
    pub fn is_playing(&self) -> bool {
        unimplemented!()
    }

    /// _Inherited from GameObject_
    ///
    /// A unique id for each instance of a GameObject or a sub class. Used for client and server
    /// communication. Should never change value after being set.
    pub fn id(&self) -> String {
        unimplemented!()
    }

    /// _Inherited from GameObject_
    ///
    /// String representing the top level Class that this game object is an instance of. Used for
    /// reflection to create new instances on clients, but exposed for convenience should AIs want
    /// this data.
    pub fn game_object_name(&self) -> String {
        unimplemented!()
    }

    /// _Inherited from GameObject_
    ///
    /// Any strings logged will be stored here. Intended for debugging.
    pub fn logs(&self) -> List<String> {
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
        _message: &String,
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
