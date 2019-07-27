#![allow(dead_code, unused_imports)]

use super::*;
use crate::util::*;

/// A checker on the game board.
pub struct Checker {
}

impl Checker {

    /// The player that controls this Checker.
    pub fn owner(&self) -> Player {
        unimplemented!()
    }

    /// The x coordinate of the checker.
    pub fn x(&self) -> isize {
        unimplemented!()
    }

    /// The y coordinate of the checker.
    pub fn y(&self) -> isize {
        unimplemented!()
    }

    /// If the checker has been kinged and can move backwards.
    pub fn kinged(&self) -> bool {
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

    /// Moves the checker from its current location to the given (x, y).
    ///
    /// # Arguments
    ///
    /// - _x_ - The x coordinate to move to.
    ///
    /// - _y_ - The y coordinate to move to.
    ///
    /// # Returns
    ///
    /// Returns the same checker that moved if the move was successful. null otherwise.
    pub fn move_to(
        &self,
        _x: isize,
        _y: isize,
    )
        -> Option<Checker>
    {
        unimplemented!()
    }

    /// Returns if the checker is owned by your player or not.
    ///
    /// # Returns
    ///
    /// True if it is yours, false if it is not yours.
    pub fn is_mine(
        &self,
    )
        -> bool
    {
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
