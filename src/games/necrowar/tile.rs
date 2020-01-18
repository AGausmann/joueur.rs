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

    /// The Tower on this Tile if present, otherwise None.
    pub fn tower(&self) -> Option<Tower> {
        unimplemented!()
    }

    /// Whether or not the tile can be moved on by workers.
    pub fn is_wall(&self) -> bool {
        unimplemented!()
    }

    /// Whether or not the tile is considered a path or not (Units can walk on paths).
    pub fn is_path(&self) -> bool {
        unimplemented!()
    }

    /// Whether or not the tile is considered grass or not (Workers can walk on grass).
    pub fn is_grass(&self) -> bool {
        unimplemented!()
    }

    /// Whether or not the tile is considered a tower or not.
    pub fn is_tower(&self) -> bool {
        unimplemented!()
    }

    /// Whether or not the tile is considered to be a gold mine or not.
    pub fn is_gold_mine(&self) -> bool {
        unimplemented!()
    }

    /// Whether or not the tile is considered to be the island gold mine or not.
    pub fn is_island_gold_mine(&self) -> bool {
        unimplemented!()
    }

    /// Whether or not the tile is considered a river or not.
    pub fn is_river(&self) -> bool {
        unimplemented!()
    }

    /// Whether or not the tile is a castle tile.
    pub fn is_castle(&self) -> bool {
        unimplemented!()
    }

    /// Whether or not the tile is the worker spawn.
    pub fn is_worker_spawn(&self) -> bool {
        unimplemented!()
    }

    /// Whether or not the tile is the unit spawn.
    pub fn is_unit_spawn(&self) -> bool {
        unimplemented!()
    }

    /// The amount of corpses on this tile.
    pub fn corpses(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of Zombies on this tile.
    pub fn num_zombies(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of Ghouls on this tile.
    pub fn num_ghouls(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of Hounds on this tile.
    pub fn num_hounds(&self) -> i64 {
        unimplemented!()
    }

    /// Which player owns this tile, only applies to grass tiles for workers, None otherwise.
    pub fn owner(&self) -> Option<Player> {
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

    /// Resurrect the corpses on this tile into Zombies.
    ///
    /// # Arguments
    ///
    /// - _num_ - Number of zombies to resurrect.
    ///
    /// # Returns
    ///
    /// True if successful res, false otherwise.
    pub fn res(
        &self,
        _num: i64,
    )
        -> Result<bool, Error>
    {
        unimplemented!()
    }

    /// Spawns a fighting unit on the correct tile.
    ///
    /// # Arguments
    ///
    /// - _title_ - The title of the desired unit type.
    ///
    /// # Returns
    ///
    /// True if successfully spawned, false otherwise.
    pub fn spawn_unit(
        &self,
        _title: &str,
    )
        -> Result<bool, Error>
    {
        unimplemented!()
    }

    /// Spawns a worker on the correct tile.
    ///
    /// # Returns
    ///
    /// True if successfully spawned, false otherwise.
    pub fn spawn_worker(
        &self,
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
