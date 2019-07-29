#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;

/// Can put out fires completely.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FireDepartment {
}

impl FireDepartment {

    /// The amount of fire removed from a building when bribed to extinguish a building.
    pub fn fire_extinguished(&self) -> i64 {
        unimplemented!()
    }

    /// _Inherited from Building_
    ///
    /// How much health this building currently has. When this reaches 0 the Building has been
    /// burned down.
    pub fn health(&self) -> i64 {
        unimplemented!()
    }

    /// _Inherited from Building_
    ///
    /// The player that owns this building. If it burns down (health reaches 0) that player gets an
    /// additional bribe(s).
    pub fn owner(&self) -> Player {
        unimplemented!()
    }

    /// _Inherited from Building_
    ///
    /// True if this is the Headquarters of the owning player, false otherwise. Burning this down
    /// wins the game for the other Player.
    pub fn is_headquarters(&self) -> bool {
        unimplemented!()
    }

    /// _Inherited from Building_
    ///
    /// When true this building has already been bribed this turn and cannot be bribed again this
    /// turn.
    pub fn bribed(&self) -> bool {
        unimplemented!()
    }

    /// _Inherited from Building_
    ///
    /// The location of the Building along the x-axis.
    pub fn x(&self) -> i64 {
        unimplemented!()
    }

    /// _Inherited from Building_
    ///
    /// The location of the Building along the y-axis.
    pub fn y(&self) -> i64 {
        unimplemented!()
    }

    /// _Inherited from Building_
    ///
    /// How much fire is currently burning the building, and thus how much damage it will take at
    /// the end of its owner's turn. 0 means no fire.
    pub fn fire(&self) -> i64 {
        unimplemented!()
    }

    /// _Inherited from Building_
    ///
    /// The Building directly to the north of this building, or None if not present.
    pub fn building_north(&self) -> Option<Building> {
        unimplemented!()
    }

    /// _Inherited from Building_
    ///
    /// The Building directly to the east of this building, or None if not present.
    pub fn building_east(&self) -> Option<Building> {
        unimplemented!()
    }

    /// _Inherited from Building_
    ///
    /// The Building directly to the south of this building, or None if not present.
    pub fn building_south(&self) -> Option<Building> {
        unimplemented!()
    }

    /// _Inherited from Building_
    ///
    /// The Building directly to the west of this building, or None if not present.
    pub fn building_west(&self) -> Option<Building> {
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

    /// Bribes this FireDepartment to extinguish the some of the fire in a building.
    ///
    /// # Arguments
    ///
    /// - _building_ - The Building you want to extinguish.
    ///
    /// # Returns
    ///
    /// True if the bribe worked, false otherwise.
    pub fn extinguish(
        &self,
        _building: &Building,
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
