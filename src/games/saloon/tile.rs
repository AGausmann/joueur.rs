#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;

/// A Tile in the game that makes up the 2D map grid.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Tile {
}

impl Tile {

    /// The x (horizontal) position of this Tile.
    pub fn x(&self) -> i64 {
        unimplemented!()
    }

    /// The y (vertical) position of this Tile.
    pub fn y(&self) -> i64 {
        unimplemented!()
    }

    /// The Tile to the 'North' of this one (x, y-1). None if out of bounds of the map.
    pub fn tile_north(&self) -> Option<Tile> {
        unimplemented!()
    }

    /// The Tile to the 'East' of this one (x+1, y). None if out of bounds of the map.
    pub fn tile_east(&self) -> Option<Tile> {
        unimplemented!()
    }

    /// The Tile to the 'South' of this one (x, y+1). None if out of bounds of the map.
    pub fn tile_south(&self) -> Option<Tile> {
        unimplemented!()
    }

    /// The Tile to the 'West' of this one (x-1, y). None if out of bounds of the map.
    pub fn tile_west(&self) -> Option<Tile> {
        unimplemented!()
    }

    /// The Cowboy that is on this Tile, None otherwise.
    pub fn cowboy(&self) -> Option<Cowboy> {
        unimplemented!()
    }

    /// The furnishing that is on this Tile, None otherwise.
    pub fn furnishing(&self) -> Option<Furnishing> {
        unimplemented!()
    }

    /// If this Tile is a balcony of the Saloon that YoungGuns walk around on, and can never be
    /// pathed through by Cowboys.
    pub fn is_balcony(&self) -> bool {
        unimplemented!()
    }

    /// If this Tile is pathable, but has a hazard that damages Cowboys that path through it.
    pub fn has_hazard(&self) -> bool {
        unimplemented!()
    }

    /// The beer Bottle currently flying over this Tile, None otherwise.
    pub fn bottle(&self) -> Option<Bottle> {
        unimplemented!()
    }

    /// The YoungGun on this tile, None otherwise.
    pub fn young_gun(&self) -> Option<YoungGun> {
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
