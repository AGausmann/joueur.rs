#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;
use crate::error::Error;

/// A Tile in the game that makes up the 2D map grid.
#[derive(Debug, Clone)]
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

    /// The Unit on this Tile if present, otherwise None.
    pub fn unit(&self) -> Option<Unit> {
        unimplemented!()
    }

    /// The Port on this Tile if present, otherwise None.
    pub fn port(&self) -> Option<Port> {
        unimplemented!()
    }

    /// The type of Tile this is ('water' or 'land').
    pub fn type_(&self) -> Str {
        unimplemented!()
    }

    /// The amount of gold buried on this tile.
    pub fn gold(&self) -> i64 {
        unimplemented!()
    }

    /// (Visualizer only) Whether this tile is deep sea or grassy. This has no effect on gameplay,
    /// but feel free to use it if you want.
    pub fn decoration(&self) -> bool {
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
