#![allow(dead_code, unused_imports)]

use super::*;
use crate::util::*;

/// A typical abandoned warehouse... that anarchists hang out in and can be bribed to burn down
/// Buildings.
pub struct Warehouse {
}

impl Warehouse {

    /// The amount of fire added to buildings when bribed to ignite a building. Headquarters add
    /// more fire than normal Warehouses.
    pub fn fire_added(&self) -> isize {
        unimplemented!()
    }

    /// How exposed the anarchists in this warehouse are to PoliceDepartments. Raises when bribed
    /// to ignite buildings, and drops each turn if not bribed.
    pub fn exposure(&self) -> isize {
        unimplemented!()
    }

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

    /// Bribes the Warehouse to light a Building on fire. This adds this building's fireAdded to
    /// their fire, and then this building's exposure is increased based on the Manhatten distance
    /// between the two buildings.
    ///
    /// # Arguments
    ///
    /// - _building_ - The Building you want to light on fire.
    ///
    /// # Returns
    ///
    /// The exposure added to this Building's exposure. -1 is returned if there was an error.
    pub fn ignite(
        &self,
        _building: &Building,
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
