#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;
use crate::error::Error;

/// A unit in the game. May be a worker, zombie, ghoul, hound, abomination, wraith or horseman.
#[derive(Debug, Clone)]
pub struct Unit {
}

impl Unit {

    /// The Player that owns and can control this Unit.
    pub fn owner(&self) -> Option<Player> {
        unimplemented!()
    }

    /// The Tile this Unit is on.
    pub fn tile(&self) -> Option<Tile> {
        unimplemented!()
    }

    /// The type of unit this is.
    pub fn job(&self) -> UnitJob {
        unimplemented!()
    }

    /// The remaining health of a unit.
    pub fn health(&self) -> i64 {
        unimplemented!()
    }

    /// Whether or not this Unit has performed its action this turn (attack or build).
    pub fn acted(&self) -> bool {
        unimplemented!()
    }

    /// The number of moves this unit has left this turn.
    pub fn moves(&self) -> i64 {
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

    /// Enters a mine and is put to work gathering resources.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The tile the mine is located on.
    ///
    /// # Returns
    ///
    /// True if successfully entered mine and began mining, false otherwise.
    pub fn mine(
        &self,
        _tile: &Tile,
    )
        -> Result<bool, Error>
    {
        unimplemented!()
    }

    /// Stops adjacent to a river tile and begins fishing for mana.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The tile the unit will stand on as it fishes.
    ///
    /// # Returns
    ///
    /// True if successfully began fishing for mana, false otherwise.
    pub fn fish(
        &self,
        _tile: &Tile,
    )
        -> Result<bool, Error>
    {
        unimplemented!()
    }

    /// Moves this Unit from its current Tile to an adjacent Tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile this Unit should move to.
    ///
    /// # Returns
    ///
    /// True if it moved, false otherwise.
    pub fn move_(
        &self,
        _tile: &Tile,
    )
        -> Result<bool, Error>
    {
        unimplemented!()
    }

    /// Attacks an enemy tower on an adjacent tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile to attack.
    ///
    /// # Returns
    ///
    /// True if successfully attacked, false otherwise.
    pub fn attack(
        &self,
        _tile: &Tile,
    )
        -> Result<bool, Error>
    {
        unimplemented!()
    }

    /// Unit, if it is a worker, builds a tower on the tile it is on, only workers can do this.
    ///
    /// # Arguments
    ///
    /// - _title_ - The tower type to build, as a string.
    ///
    /// # Returns
    ///
    /// True if successfully built, false otherwise.
    pub fn build(
        &self,
        _title: &str,
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
