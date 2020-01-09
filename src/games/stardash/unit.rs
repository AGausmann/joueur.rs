#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;

/// A unit in the game. May be a corvette, missleboat, martyr, transport, miner.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
}

impl Unit {

    /// The Player that owns and can control this Unit.
    pub fn owner(&self) -> Option<Player> {
        unimplemented!()
    }

    /// The x value this unit is on.
    pub fn x(&self) -> f64 {
        unimplemented!()
    }

    /// The y value this unit is on.
    pub fn y(&self) -> f64 {
        unimplemented!()
    }

    /// The x value this unit is dashing to.
    pub fn dash_x(&self) -> f64 {
        unimplemented!()
    }

    /// The y value this unit is dashing to.
    pub fn dash_y(&self) -> f64 {
        unimplemented!()
    }

    /// The Job this Unit has.
    pub fn job(&self) -> Job {
        unimplemented!()
    }

    /// The remaining health of the unit.
    pub fn energy(&self) -> i64 {
        unimplemented!()
    }

    /// The sheild that a martyr ship has.
    pub fn shield(&self) -> i64 {
        unimplemented!()
    }

    /// Whether or not this Unit has performed its action this turn.
    pub fn acted(&self) -> bool {
        unimplemented!()
    }

    /// The distance this unit can still move.
    pub fn moves(&self) -> f64 {
        unimplemented!()
    }

    /// The amount of Genarium ore carried by this unit. (0 to job carry capacity - other carried
    /// items).
    pub fn genarium(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of Rarium carried by this unit. (0 to job carry capacity - other carried items).
    pub fn rarium(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of Legendarium ore carried by this unit. (0 to job carry capacity - other
    /// carried items).
    pub fn legendarium(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of Mythicite carried by this unit. (0 to job carry capacity - other carried
    /// items).
    pub fn mythicite(&self) -> i64 {
        unimplemented!()
    }

    /// Tracks wheither or not the ship is dashing or Mining. If true, it cannot do anything else.
    pub fn is_busy(&self) -> bool {
        unimplemented!()
    }

    /// The martyr ship that is currently shielding this ship if any.
    pub fn protector(&self) -> Option<Unit> {
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

    /// Grab materials from a friendly unit. Doesn't use a action.
    ///
    /// # Arguments
    ///
    /// - _unit_ - The unit you are grabbing the resources from.
    ///
    /// - _amount_ - The amount of materials to you with to grab. Amounts <= 0 will pick up all the
    /// materials that the unit can.
    ///
    /// - _material_ - The material the unit will pick up. 'genarium', 'rarium', 'legendarium', or
    /// 'mythicite'.
    ///
    /// # Returns
    ///
    /// True if successfully taken, false otherwise.
    pub fn transfer(
        &self,
        _unit: &Unit,
        _amount: i64,
        _material: &str,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Moves this Unit from its current location to the new location specified.
    ///
    /// # Arguments
    ///
    /// - _x_ - The x value of the destination's coordinates.
    ///
    /// - _y_ - The y value of the destination's coordinates.
    ///
    /// # Returns
    ///
    /// True if it moved, false otherwise.
    pub fn move_to(
        &self,
        _x: f64,
        _y: f64,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Attacks the specified unit.
    ///
    /// # Arguments
    ///
    /// - _enemy_ - The Unit being attacked.
    ///
    /// # Returns
    ///
    /// True if successfully attacked, false otherwise.
    pub fn attack(
        &self,
        _enemy: &Unit,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Attacks the specified projectile.
    ///
    /// # Arguments
    ///
    /// - _missile_ - The projectile being shot down.
    ///
    /// # Returns
    ///
    /// True if successfully attacked, false otherwise.
    pub fn shootdown(
        &self,
        _missile: &Projectile,
    )
        -> bool
    {
        unimplemented!()
    }

    /// allows a miner to mine a asteroid
    ///
    /// # Arguments
    ///
    /// - _body_ - The object to be mined.
    ///
    /// # Returns
    ///
    /// True if successfully acted, false otherwise.
    pub fn mine(
        &self,
        _body: &Body,
    )
        -> bool
    {
        unimplemented!()
    }

    /// tells you if your ship can move to that location from were it is without clipping the sun.
    ///
    /// # Arguments
    ///
    /// - _x_ - The x position of the location you wish to arrive.
    ///
    /// - _y_ - The y position of the location you wish to arrive.
    ///
    /// # Returns
    ///
    /// True if pathable by this unit, false otherwise.
    pub fn safe(
        &self,
        _x: f64,
        _y: f64,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Causes the unit to dash towards the designated destination.
    ///
    /// # Arguments
    ///
    /// - _x_ - The x value of the destination's coordinates.
    ///
    /// - _y_ - The y value of the destination's coordinates.
    ///
    /// # Returns
    ///
    /// True if it moved, false otherwise.
    pub fn dash(
        &self,
        _x: f64,
        _y: f64,
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
