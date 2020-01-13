#![allow(dead_code, unused_imports)]

use std::sync::{Arc, Mutex, Weak};
use std::cell::{RefCell, RefMut};

use super::*;
use crate::types::*;

/// A unit in the game.
#[derive(Debug, Clone)]
pub struct Unit {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<UnitInner>>,
}

#[derive(Debug, Clone)]
struct UnitInner {
    unit: Arc<Mutex<UnitBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct UnitBase {
    pub(crate) owner: Option<Player>,
    pub(crate) tile: Option<Tile>,
    pub(crate) job: Job,
    pub(crate) moves: i64,
    pub(crate) energy: f64,
    pub(crate) squad: List<Unit>,
    pub(crate) acted: bool,
    pub(crate) food: i64,
    pub(crate) materials: i64,
    pub(crate) starving: bool,
    pub(crate) turns_to_die: i64,
    pub(crate) movement_target: Option<Tile>,
}

impl Unit {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<UnitInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Unit = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The Player that owns and can control this Unit, or None if the Unit is neutral.
    pub fn owner(&self) -> Option<Player> {
        self.inner().unit.lock().unwrap().owner.clone()
    }

    /// The Tile this Unit is on.
    pub fn tile(&self) -> Option<Tile> {
        self.inner().unit.lock().unwrap().tile.clone()
    }

    /// The Job this Unit was recruited to do.
    pub fn job(&self) -> Job {
        self.inner().unit.lock().unwrap().job.clone()
    }

    /// How many moves this Unit has left this turn.
    pub fn moves(&self) -> i64 {
        self.inner().unit.lock().unwrap().moves.clone()
    }

    /// The amount of energy this Unit has (from 0.0 to 100.0).
    pub fn energy(&self) -> f64 {
        self.inner().unit.lock().unwrap().energy.clone()
    }

    /// The Units in the same squad as this Unit. Units in the same squad attack and defend
    /// together.
    pub fn squad(&self) -> List<Unit> {
        self.inner().unit.lock().unwrap().squad.clone()
    }

    /// Whether this Unit has performed its action this turn.
    pub fn acted(&self) -> bool {
        self.inner().unit.lock().unwrap().acted.clone()
    }

    /// The amount of food this Unit is holding.
    pub fn food(&self) -> i64 {
        self.inner().unit.lock().unwrap().food.clone()
    }

    /// The amount of materials this Unit is holding.
    pub fn materials(&self) -> i64 {
        self.inner().unit.lock().unwrap().materials.clone()
    }

    /// Whether this Unit is starving. Starving Units regenerate energy at half the rate they
    /// normally would while resting.
    pub fn starving(&self) -> bool {
        self.inner().unit.lock().unwrap().starving.clone()
    }

    /// The number of turns before this Unit dies. This only applies to neutral fresh humans
    /// created from combat. Otherwise, 0.
    pub fn turns_to_die(&self) -> i64 {
        self.inner().unit.lock().unwrap().turns_to_die.clone()
    }

    /// The tile this Unit is moving to. This only applies to neutral fresh humans spawned on the
    /// road. Otherwise, the tile this Unit is on.
    pub fn movement_target(&self) -> Option<Tile> {
        self.inner().unit.lock().unwrap().movement_target.clone()
    }

    /// _Inherited from GameObject_
    ///
    /// A unique id for each instance of a GameObject or a sub class. Used for client and server
    /// communication. Should never change value after being set.
    pub fn id(&self) -> Str {
        self.inner().game_object.lock().unwrap().id.clone()
    }

    /// _Inherited from GameObject_
    ///
    /// String representing the top level Class that this game object is an instance of. Used for
    /// reflection to create new instances on clients, but exposed for convenience should AIs want
    /// this data.
    pub fn game_object_name(&self) -> Str {
        self.inner().game_object.lock().unwrap().game_object_name.clone()
    }

    /// _Inherited from GameObject_
    ///
    /// Any strings logged will be stored here. Intended for debugging.
    pub fn logs(&self) -> List<Str> {
        self.inner().game_object.lock().unwrap().logs.clone()
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
        -> bool
    {
        unimplemented!()
    }

    /// Harvests the food on an adjacent Tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile you want to harvest.
    ///
    /// # Returns
    ///
    /// True if successfully harvested, false otherwise.
    pub fn harvest(
        &self,
        _tile: &Tile,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Attacks an adjacent Tile. Costs an action for each Unit in this Unit's squad. Units in the
    /// squad without an action don't participate in combat. Units in combat cannot move
    /// afterwards. Attacking structures will not give materials.
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

    /// Converts an adjacent Unit to your side.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile with the Unit to convert.
    ///
    /// # Returns
    ///
    /// True if successfully converted, false otherwise.
    pub fn convert(
        &self,
        _tile: &Tile,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Constructs a Structure on an adjacent Tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile to construct the Structure on. It must have enough materials on it for
    /// a Structure to be constructed.
    ///
    /// - _type__ - The type of Structure to construct on that Tile.
    ///
    /// # Returns
    ///
    /// True if successfully constructed a structure, false otherwise.
    pub fn construct(
        &self,
        _tile: &Tile,
        _type_: &str,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Removes materials from an adjacent Tile's Structure. You cannot deconstruct friendly
    /// structures (see Unit.attack).
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile to deconstruct. It must have a Structure on it.
    ///
    /// # Returns
    ///
    /// True if successfully deconstructed, false otherwise.
    pub fn deconstruct(
        &self,
        _tile: &Tile,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Drops some of the given resource on or adjacent to the Unit's Tile. Does not count as an
    /// action.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile to drop materials/food on.
    ///
    /// - _resource_ - The type of resource to drop ('materials' or 'food').
    ///
    /// - _amount_ - The amount of the resource to drop. Amounts <= 0 will drop as much as
    /// possible.
    ///
    /// # Returns
    ///
    /// True if successfully dropped the resource, false otherwise.
    pub fn drop(
        &self,
        _tile: &Tile,
        _resource: &str,
        _amount: i64,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Picks up some materials or food on or adjacent to the Unit's Tile. Does not count as an
    /// action.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile to pickup materials/food from.
    ///
    /// - _resource_ - The type of resource to pickup ('materials' or 'food').
    ///
    /// - _amount_ - The amount of the resource to pickup. Amounts <= 0 will pickup as much as
    /// possible.
    ///
    /// # Returns
    ///
    /// True if successfully picked up a resource, false otherwise.
    pub fn pickup(
        &self,
        _tile: &Tile,
        _resource: &str,
        _amount: i64,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Changes this Unit's Job. Must be at max energy (100.0) to change Jobs.
    ///
    /// # Arguments
    ///
    /// - _job_ - The name of the Job to change to.
    ///
    /// # Returns
    ///
    /// True if successfully changed Jobs, false otherwise.
    pub fn change_job(
        &self,
        _job: &str,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Regenerates energy. Must be in range of a friendly shelter to rest. Costs an action. Units
    /// cannot move after resting.
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

    pub fn try_cast<T>(&self) -> Option<T> {
        self.context().try_get_obj(&self.id)
    }

    pub fn cast<T>(&self) -> Option<T> {
        self.context().get_obj(&self.id)
    }
}
