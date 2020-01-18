#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;
use crate::error::Error;

/// Can be bribed to change the next Forecast in some way.
#[derive(Debug, Clone)]
pub struct WeatherStation {
}

impl WeatherStation {

    /// _Inherited from [`Building`]_
    ///
    /// How much health this building currently has. When this reaches 0 the Building has been
    /// burned down.
    pub fn health(&self) -> i64 {
        unimplemented!()
    }

    /// _Inherited from [`Building`]_
    ///
    /// The player that owns this building. If it burns down (health reaches 0) that player gets an
    /// additional bribe(s).
    pub fn owner(&self) -> Player {
        unimplemented!()
    }

    /// _Inherited from [`Building`]_
    ///
    /// True if this is the Headquarters of the owning player, false otherwise. Burning this down
    /// wins the game for the other Player.
    pub fn is_headquarters(&self) -> bool {
        unimplemented!()
    }

    /// _Inherited from [`Building`]_
    ///
    /// When true this building has already been bribed this turn and cannot be bribed again this
    /// turn.
    pub fn bribed(&self) -> bool {
        unimplemented!()
    }

    /// _Inherited from [`Building`]_
    ///
    /// The location of the Building along the x-axis.
    pub fn x(&self) -> i64 {
        unimplemented!()
    }

    /// _Inherited from [`Building`]_
    ///
    /// The location of the Building along the y-axis.
    pub fn y(&self) -> i64 {
        unimplemented!()
    }

    /// _Inherited from [`Building`]_
    ///
    /// How much fire is currently burning the building, and thus how much damage it will take at
    /// the end of its owner's turn. 0 means no fire.
    pub fn fire(&self) -> i64 {
        unimplemented!()
    }

    /// _Inherited from [`Building`]_
    ///
    /// The Building directly to the north of this building, or None if not present.
    pub fn building_north(&self) -> Option<Building> {
        unimplemented!()
    }

    /// _Inherited from [`Building`]_
    ///
    /// The Building directly to the east of this building, or None if not present.
    pub fn building_east(&self) -> Option<Building> {
        unimplemented!()
    }

    /// _Inherited from [`Building`]_
    ///
    /// The Building directly to the south of this building, or None if not present.
    pub fn building_south(&self) -> Option<Building> {
        unimplemented!()
    }

    /// _Inherited from [`Building`]_
    ///
    /// The Building directly to the west of this building, or None if not present.
    pub fn building_west(&self) -> Option<Building> {
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

    /// Bribe the weathermen to change the direction of the next Forecast by rotating it clockwise
    /// or counterclockwise.
    ///
    /// # Arguments
    ///
    /// - _counterclockwise_ - By default the direction will be rotated clockwise. If you set this
    /// to true we will rotate the forecast counterclockwise instead.
    ///
    /// # Returns
    ///
    /// True if the rotation worked, false otherwise.
    pub fn rotate(
        &self,
        _counterclockwise: bool,
    )
        -> Result<bool, Error>
    {
        unimplemented!()
    }

    /// Bribe the weathermen to intensity the next Forecast by 1 or -1
    ///
    /// # Arguments
    ///
    /// - _negative_ - By default the intensity will be increased by 1, setting this to true
    /// decreases the intensity by 1.
    ///
    /// # Returns
    ///
    /// True if the intensity was changed, false otherwise.
    pub fn intensify(
        &self,
        _negative: bool,
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
