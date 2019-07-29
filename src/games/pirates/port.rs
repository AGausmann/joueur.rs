#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;

/// A port on a Tile.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Port {
}

impl Port {

    /// The Tile this Port is on.
    pub fn tile(&self) -> Tile {
        unimplemented!()
    }

    /// The owner of this Port, or None if owned by merchants.
    pub fn owner(&self) -> Option<Player> {
        unimplemented!()
    }

    /// For players, how much more gold this Port can spend this turn. For merchants, how much gold
    /// this Port has accumulated (it will spawn a ship when the Port can afford one).
    pub fn gold(&self) -> i64 {
        unimplemented!()
    }

    /// (Merchants only) How much gold was invested into this Port. Investment determines the
    /// strength and value of the next ship.
    pub fn investment(&self) -> i64 {
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

    /// Spawn a Unit on this port.
    ///
    /// # Arguments
    ///
    /// - _type_of_ - What type of Unit to create ('crew' or 'ship').
    ///
    /// # Returns
    ///
    /// True if Unit was created successfully, false otherwise.
    pub fn spawn(
        &self,
        _type_of: &str,
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
