#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;

/// A tower in the game. Used to combat enemy waves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tower {
}

impl Tower {

    /// The player that built / owns this tower.
    pub fn owner(&self) -> Option<Player> {
        unimplemented!()
    }

    /// The Tile this Tower is on.
    pub fn tile(&self) -> Tile {
        unimplemented!()
    }

    /// What type of tower this is (it's job).
    pub fn job(&self) -> TowerJob {
        unimplemented!()
    }

    /// How much remaining health this tower has.
    pub fn health(&self) -> i64 {
        unimplemented!()
    }

    /// Whether this tower has attacked this turn or not.
    pub fn attacked(&self) -> bool {
        unimplemented!()
    }

    /// How many turns are left before it can fire again.
    pub fn cooldown(&self) -> i64 {
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

    /// Attacks an enemy unit on an tile within it's range.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile to attack.
    ///
    /// # Returns
    ///
    /// True if successfully attacked, false otherwise.
    pub fn attack(
        &self,
        _tile: &Tile,
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
