#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;

/// Information about a unit's job/type.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnitJob {
}

impl UnitJob {

    /// The type title. 'worker', 'zombie', 'ghoul', 'hound', 'abomination', 'wraith' or
    /// 'horseman'.
    pub fn title(&self) -> Str {
        unimplemented!()
    }

    /// How many of this type of unit can take up one tile.
    pub fn per_tile(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of starting health this type has.
    pub fn health(&self) -> i64 {
        unimplemented!()
    }

    /// The number of moves this type can make per turn.
    pub fn moves(&self) -> i64 {
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

    /// How much does this type cost in mana.
    pub fn mana_cost(&self) -> i64 {
        unimplemented!()
    }

    /// Amount of tiles away this type has to be in order to be effective.
    pub fn range(&self) -> i64 {
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
