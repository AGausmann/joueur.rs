#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;

/// Information about a tower's job/type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TowerJob {
}

impl TowerJob {

    /// The type title. 'arrow', 'aoe', 'ballista', 'cleansing', or 'castle'.
    pub fn title(&self) -> Str {
        unimplemented!()
    }

    /// The amount of starting health this type has.
    pub fn health(&self) -> i64 {
        unimplemented!()
    }

    /// The number of tiles this type can attack from.
    pub fn range(&self) -> i64 {
        unimplemented!()
    }

    /// Whether this tower type hits all of the units on a tile (true) or one at a time (false).
    pub fn all_units(&self) -> bool {
        unimplemented!()
    }

    /// The amount of damage this type does per attack.
    pub fn damage(&self) -> i64 {
        unimplemented!()
    }

    /// How much does this type cost in gold.
    pub fn gold_cost(&self) -> i64 {
        unimplemented!()
    }

    /// How many turns have to take place between this type's attacks.
    pub fn turns_between_attacks(&self) -> i64 {
        unimplemented!()
    }

    /// How much does this type cost in mana.
    pub fn mana_cost(&self) -> i64 {
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
