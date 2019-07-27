#![allow(dead_code, unused_imports)]

use super::*;
use crate::util::*;

/// Used to keep cities under control and raid Warehouses.
pub struct PoliceDepartment {
}

impl PoliceDepartment {

    /// _Inherited from Building_
    ///
    /// How much health this building currently has. When this reaches 0 the Building has been
    /// burned down.
    pub fn health(&self) -> isize {
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
    pub fn x(&self) -> isize {
        unimplemented!()
    }

    /// _Inherited from Building_
    ///
    /// The location of the Building along the y-axis.
    pub fn y(&self) -> isize {
        unimplemented!()
    }

    /// _Inherited from Building_
    ///
    /// How much fire is currently burning the building, and thus how much damage it will take at
    /// the end of its owner's turn. 0 means no fire.
    pub fn fire(&self) -> isize {
        unimplemented!()
    }

    /// _Inherited from Building_
    ///
    /// The Building directly to the north of this building, or null if not present.
    pub fn building_north(&self) -> Option<Building> {
        unimplemented!()
    }

    /// _Inherited from Building_
    ///
    /// The Building directly to the east of this building, or null if not present.
    pub fn building_east(&self) -> Option<Building> {
        unimplemented!()
    }

    /// _Inherited from Building_
    ///
    /// The Building directly to the south of this building, or null if not present.
    pub fn building_south(&self) -> Option<Building> {
        unimplemented!()
    }

    /// _Inherited from Building_
    ///
    /// The Building directly to the west of this building, or null if not present.
    pub fn building_west(&self) -> Option<Building> {
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

    /// Bribe the police to raid a Warehouse, dealing damage equal based on the Warehouse's current
    /// exposure, and then resetting it to 0.
    ///
    /// # Arguments
    ///
    /// - _warehouse_ - The warehouse you want to raid.
    ///
    /// # Returns
    ///
    /// The amount of damage dealt to the warehouse, or -1 if there was an error.
    pub fn raid(
        &self,
        _warehouse: &Warehouse,
    )
        -> isize
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
