#![allow(dead_code, unused_imports)]

use super::*;
use crate::util::*;

/// An eager young person that wants to join your gang, and will call in the veteran Cowboys you
/// need to win the brawl in the saloon.
pub struct YoungGun {
}

impl YoungGun {

    /// The Player that owns and can control this YoungGun.
    pub fn owner(&self) -> Player {
        unimplemented!()
    }

    /// The Tile this YoungGun is currently on.
    pub fn tile(&self) -> Tile {
        unimplemented!()
    }

    /// True if the YoungGun can call in a Cowboy, false otherwise.
    pub fn can_call_in(&self) -> bool {
        unimplemented!()
    }

    /// The Tile that a Cowboy will be called in on if this YoungGun calls in a Cowboy.
    pub fn call_in_tile(&self) -> Tile {
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

    /// Tells the YoungGun to call in a new Cowboy of the given job to the open Tile nearest to
    /// them.
    ///
    /// # Arguments
    ///
    /// - _job_ - The job you want the Cowboy being brought to have.
    ///
    /// # Returns
    ///
    /// The new Cowboy that was called in if valid. They will not be added to any `cowboys` lists
    /// until the turn ends. Null otherwise.
    pub fn call_in(
        &self,
        _job: &String,
    )
        -> Option<Cowboy>
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
