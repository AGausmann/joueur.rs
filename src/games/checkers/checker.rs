#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;
use crate::error::Error;

/// A checker on the game board.
#[derive(Debug, Clone)]
pub struct Checker {
}

impl Checker {

    /// The player that controls this Checker.
    pub fn owner(&self) -> Player {
        unimplemented!()
    }

    /// The x coordinate of the checker.
    pub fn x(&self) -> i64 {
        unimplemented!()
    }

    /// The y coordinate of the checker.
    pub fn y(&self) -> i64 {
        unimplemented!()
    }

    /// If the checker has been kinged and can move backwards.
    pub fn kinged(&self) -> bool {
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
    /// Returns the same checker that moved if the move was successful. None otherwise.
    pub fn move_(
        &self,
        _x: i64,
        _y: i64,
    )
        -> Result<Option<Checker>, Error>
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
        -> Result<bool, Error>
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
