#![allow(dead_code, unused_imports)]

use super::*;
use crate::util::*;

/// A unit in the game. May be a manager, intern, or physicist.
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

    /// The Job this Unit has.
    pub fn job(&self) -> Job {
        unimplemented!()
    }

    /// The remaining health of a unit.
    pub fn health(&self) -> isize {
        unimplemented!()
    }

    /// Whether or not this Unit has performed its action this turn.
    pub fn acted(&self) -> bool {
        unimplemented!()
    }

    /// The number of moves this unit has left this turn.
    pub fn moves(&self) -> isize {
        unimplemented!()
    }

    /// The amount of redium ore carried by this unit. (0 to job carry capacity - other carried
    /// items).
    pub fn redium_ore(&self) -> isize {
        unimplemented!()
    }

    /// The amount of redium carried by this unit. (0 to job carry capacity - other carried items).
    pub fn redium(&self) -> isize {
        unimplemented!()
    }

    /// The amount of blueium ore carried by this unit. (0 to job carry capacity - other carried
    /// items).
    pub fn blueium_ore(&self) -> isize {
        unimplemented!()
    }

    /// The amount of blueium carried by this unit. (0 to job carry capacity - other carried
    /// items).
    pub fn blueium(&self) -> isize {
        unimplemented!()
    }

    /// Duration the unit is stunned. (0 to the game constant stunTime).
    pub fn stun_time(&self) -> isize {
        unimplemented!()
    }

    /// Duration of stun immunity. (0 to timeImmune).
    pub fn stun_immune(&self) -> isize {
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

    /// Drops materials at the units feet or adjacent tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The tile the materials will be dropped on.
    ///
    /// - _amount_ - The number of materials to dropped. Amounts <= 0 will drop all the materials.
    ///
    /// - _material_ - The material the unit will drop. 'redium', 'blueium', 'redium ore', or
    /// 'blueium ore'.
    ///
    /// # Returns
    ///
    /// True if successfully deposited, false otherwise.
    pub fn drop(
        &self,
        _tile: &Tile,
        _amount: isize,
        _material: &String,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Picks up material at the units feet or adjacent tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The tile the materials will be picked up from.
    ///
    /// - _amount_ - The amount of materials to pick up. Amounts <= 0 will pick up all the
    /// materials that the unit can.
    ///
    /// - _material_ - The material the unit will pick up. 'redium', 'blueium', 'redium ore', or
    /// 'blueium ore'.
    ///
    /// # Returns
    ///
    /// True if successfully deposited, false otherwise.
    pub fn pickup(
        &self,
        _tile: &Tile,
        _amount: isize,
        _material: &String,
    )
        -> bool
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
    pub fn move_to(
        &self,
        _tile: &Tile,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Attacks a unit on an adjacent tile.
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
        -> bool
    {
        unimplemented!()
    }

    /// Makes the unit do something to a machine or unit adjacent to its tile. Interns sabotage,
    /// physicists work. Interns stun physicist, physicist stuns manager, manager stuns intern.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The tile the unit acts on.
    ///
    /// # Returns
    ///
    /// True if successfully acted, false otherwise.
    pub fn act(
        &self,
        _tile: &Tile,
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
