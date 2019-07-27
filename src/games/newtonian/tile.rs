#![allow(dead_code, unused_imports)]

use super::*;
use crate::util::*;

/// A Tile in the game that makes up the 2D map grid.
pub struct Tile {
}

impl Tile {

    /// The x (horizontal) position of this Tile.
    pub fn x(&self) -> isize {
        unimplemented!()
    }

    /// The y (vertical) position of this Tile.
    pub fn y(&self) -> isize {
        unimplemented!()
    }

    /// The Tile to the 'North' of this one (x, y-1). Null if out of bounds of the map.
    pub fn tile_north(&self) -> Option<Tile> {
        unimplemented!()
    }

    /// The Tile to the 'East' of this one (x+1, y). Null if out of bounds of the map.
    pub fn tile_east(&self) -> Option<Tile> {
        unimplemented!()
    }

    /// The Tile to the 'South' of this one (x, y+1). Null if out of bounds of the map.
    pub fn tile_south(&self) -> Option<Tile> {
        unimplemented!()
    }

    /// The Tile to the 'West' of this one (x-1, y). Null if out of bounds of the map.
    pub fn tile_west(&self) -> Option<Tile> {
        unimplemented!()
    }

    /// The Unit on this Tile if present, otherwise null.
    pub fn unit(&self) -> Option<Unit> {
        unimplemented!()
    }

    /// The Machine on this Tile if present, otherwise null.
    pub fn machine(&self) -> Option<Machine> {
        unimplemented!()
    }

    /// Whether or not the tile is a wall.
    pub fn is_wall(&self) -> bool {
        unimplemented!()
    }

    /// The type of Tile this is ('normal', 'generator', 'conveyor', or 'spawn').
    pub fn type_of(&self) -> String {
        unimplemented!()
    }

    /// The direction of a conveyor belt ('blank', 'north', 'east', 'south', or 'west'). blank
    /// means conveyor doesn't move.
    pub fn direction(&self) -> String {
        unimplemented!()
    }

    /// The owner of this Tile, or null if owned by no-one. Only for generators and spawn areas.
    pub fn owner(&self) -> Option<Player> {
        unimplemented!()
    }

    /// The amount of redium ore on this tile.
    pub fn redium_ore(&self) -> isize {
        unimplemented!()
    }

    /// The amount of redium on this tile.
    pub fn redium(&self) -> isize {
        unimplemented!()
    }

    /// The amount of blueium ore on this tile.
    pub fn blueium_ore(&self) -> isize {
        unimplemented!()
    }

    /// The amount of blueium on this tile.
    pub fn blueium(&self) -> isize {
        unimplemented!()
    }

    /// (Visualizer only) Different tile types, cracked, slightly dirty, etc. This has no effect on
    /// gameplay, but feel free to use it if you want.
    pub fn decoration(&self) -> isize {
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
