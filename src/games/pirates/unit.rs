#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;

/// A unit group in the game. This may consist of a ship and any number of crew.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unit {
}

impl Unit {

    /// The Player that owns and can control this Unit, or None if the Unit is neutral.
    pub fn owner(&self) -> Option<Player> {
        unimplemented!()
    }

    /// The Tile this Unit is on.
    pub fn tile(&self) -> Option<Tile> {
        unimplemented!()
    }

    /// If a ship is on this Tile, how much health it has remaining. 0 for no ship.
    pub fn ship_health(&self) -> i64 {
        unimplemented!()
    }

    /// How many crew are on this Tile. This number will always be <= crewHealth.
    pub fn crew(&self) -> i64 {
        unimplemented!()
    }

    /// How much total health the crew on this Tile have.
    pub fn crew_health(&self) -> i64 {
        unimplemented!()
    }

    /// How much gold this Unit is carrying.
    pub fn gold(&self) -> i64 {
        unimplemented!()
    }

    /// Whether this Unit has performed its action this turn.
    pub fn acted(&self) -> bool {
        unimplemented!()
    }

    /// How many more times this Unit may move this turn.
    pub fn moves(&self) -> i64 {
        unimplemented!()
    }

    /// (Merchants only) The path this Unit will follow. The first element is the Tile this Unit
    /// will move to next.
    pub fn path(&self) -> List<Tile> {
        unimplemented!()
    }

    /// (Merchants only) The Port this Unit is moving to.
    pub fn target_port(&self) -> Option<Port> {
        unimplemented!()
    }

    /// (Merchants only) The number of turns this merchant ship won't be able to move. They will
    /// still attack. Merchant ships are stunned when they're attacked.
    pub fn stun_turns(&self) -> i64 {
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

    /// Moves this Unit from its current Tile to an adjacent Tile. If this Unit merges with another
    /// one, the other Unit will be destroyed and its tile will be set to None. Make sure to check
    /// that your Unit's tile is not None before doing things with it.
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

    /// Attacks either the 'crew' or 'ship' on a Tile in range.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile to attack.
    ///
    /// - _target_ - Whether to attack 'crew' or 'ship'. Crew deal damage to crew and ships deal
    /// damage to ships. Consumes any remaining moves.
    ///
    /// # Returns
    ///
    /// True if successfully attacked, false otherwise.
    pub fn attack(
        &self,
        _tile: &Tile,
        _target: &str,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Buries gold on this Unit's Tile. Gold must be a certain distance away for it to get
    /// interest (Game.minInterestDistance).
    ///
    /// # Arguments
    ///
    /// - _amount_ - How much gold this Unit should bury. Amounts <= 0 will bury as much as
    /// possible.
    ///
    /// # Returns
    ///
    /// True if successfully buried, false otherwise.
    pub fn bury(
        &self,
        _amount: i64,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Digs up gold on this Unit's Tile.
    ///
    /// # Arguments
    ///
    /// - _amount_ - How much gold this Unit should take. Amounts <= 0 will dig up as much as
    /// possible.
    ///
    /// # Returns
    ///
    /// True if successfully dug up, false otherwise.
    pub fn dig(
        &self,
        _amount: i64,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Puts gold into an adjacent Port. If that Port is the Player's port, the gold is added to
    /// that Player. If that Port is owned by merchants, it adds to that Port's investment.
    ///
    /// # Arguments
    ///
    /// - _amount_ - The amount of gold to deposit. Amounts <= 0 will deposit all the gold on this
    /// Unit.
    ///
    /// # Returns
    ///
    /// True if successfully deposited, false otherwise.
    pub fn deposit(
        &self,
        _amount: i64,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Takes gold from the Player. You can only withdraw from your own Port.
    ///
    /// # Arguments
    ///
    /// - _amount_ - The amount of gold to withdraw. Amounts <= 0 will withdraw everything.
    ///
    /// # Returns
    ///
    /// True if successfully withdrawn, false otherwise.
    pub fn withdraw(
        &self,
        _amount: i64,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Moves a number of crew from this Unit to the given Tile. This will consume a move from
    /// those crew.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile to move the crew to.
    ///
    /// - _amount_ - The number of crew to move onto that Tile. Amount <= 0 will move all the crew
    /// to that Tile.
    ///
    /// - _gold_ - The amount of gold the crew should take with them. Gold < 0 will move all the
    /// gold to that Tile.
    ///
    /// # Returns
    ///
    /// True if successfully split, false otherwise.
    pub fn split(
        &self,
        _tile: &Tile,
        _amount: i64,
        _gold: i64,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Regenerates this Unit's health. Must be used in range of a port.
    ///
    /// # Returns
    ///
    /// True if successfully rested, false otherwise.
    pub fn rest(
        &self,
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
