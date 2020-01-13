#![allow(dead_code, unused_imports)]

use std::sync::{Arc, Mutex, Weak};
use std::cell::{RefCell, RefMut};

use super::*;
use crate::types::*;

/// A beaver in the game.
#[derive(Debug, Clone)]
pub struct Beaver {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<BeaverInner>>,
}

#[derive(Debug, Clone)]
struct BeaverInner {
    beaver: Arc<Mutex<BeaverBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct BeaverBase {
    pub(crate) moves: i64,
    pub(crate) owner: Player,
    pub(crate) actions: i64,
    pub(crate) tile: Option<Tile>,
    pub(crate) health: i64,
    pub(crate) turns_distracted: i64,
    pub(crate) branches: i64,
    pub(crate) food: i64,
    pub(crate) job: Job,
    pub(crate) recruited: bool,
}

impl Beaver {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<BeaverInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Beaver = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// How many moves this Beaver has left this turn.
    pub fn moves(&self) -> i64 {
        self.inner().beaver.lock().unwrap().moves.clone()
    }

    /// The Player that owns and can control this Beaver.
    pub fn owner(&self) -> Player {
        self.inner().beaver.lock().unwrap().owner.clone()
    }

    /// The number of actions remaining for the Beaver this turn.
    pub fn actions(&self) -> i64 {
        self.inner().beaver.lock().unwrap().actions.clone()
    }

    /// The Tile this Beaver is on.
    pub fn tile(&self) -> Option<Tile> {
        self.inner().beaver.lock().unwrap().tile.clone()
    }

    /// How much health this Beaver has left.
    pub fn health(&self) -> i64 {
        self.inner().beaver.lock().unwrap().health.clone()
    }

    /// Number of turns this Beaver is distracted for (0 means not distracted).
    pub fn turns_distracted(&self) -> i64 {
        self.inner().beaver.lock().unwrap().turns_distracted.clone()
    }

    /// The amount of branches this Beaver is holding.
    pub fn branches(&self) -> i64 {
        self.inner().beaver.lock().unwrap().branches.clone()
    }

    /// The amount of food this Beaver is holding.
    pub fn food(&self) -> i64 {
        self.inner().beaver.lock().unwrap().food.clone()
    }

    /// The Job this Beaver was recruited to do.
    pub fn job(&self) -> Job {
        self.inner().beaver.lock().unwrap().job.clone()
    }

    /// True if the Beaver has finished being recruited and can do things, False otherwise.
    pub fn recruited(&self) -> bool {
        self.inner().beaver.lock().unwrap().recruited.clone()
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

    /// Moves this Beaver from its current Tile to an adjacent Tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile this Beaver should move to.
    ///
    /// # Returns
    ///
    /// True if the move worked, false otherwise.
    pub fn move_(
        &self,
        _tile: &Tile,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Harvests the branches or food from a Spawner on an adjacent Tile.
    ///
    /// # Arguments
    ///
    /// - _spawner_ - The Spawner you want to harvest. Must be on an adjacent Tile.
    ///
    /// # Returns
    ///
    /// True if successfully harvested, false otherwise.
    pub fn harvest(
        &self,
        _spawner: &Spawner,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Attacks another adjacent beaver.
    ///
    /// # Arguments
    ///
    /// - _beaver_ - The Beaver to attack. Must be on an adjacent Tile.
    ///
    /// # Returns
    ///
    /// True if successfully attacked, false otherwise.
    pub fn attack(
        &self,
        _beaver: &Beaver,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Builds a lodge on the Beavers current Tile.
    ///
    /// # Returns
    ///
    /// True if successfully built a lodge, false otherwise.
    pub fn build_lodge(
        &self,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Drops some of the given resource on the beaver's Tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile to drop branches/food on. Must be the same Tile that the Beaver is on,
    /// or an adjacent one.
    ///
    /// - _resource_ - The type of resource to drop ('branches' or 'food').
    ///
    /// - _amount_ - The amount of the resource to drop, numbers <= 0 will drop all the resource
    /// type.
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

    /// Picks up some branches or food on the beaver's tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile to pickup branches/food from. Must be the same Tile that the Beaver is
    /// on, or an adjacent one.
    ///
    /// - _resource_ - The type of resource to pickup ('branches' or 'food').
    ///
    /// - _amount_ - The amount of the resource to drop, numbers <= 0 will pickup all of the
    /// resource type.
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
