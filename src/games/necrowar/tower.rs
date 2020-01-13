#![allow(dead_code, unused_imports)]

use std::sync::{Arc, Mutex, Weak};
use std::cell::{RefCell, RefMut};

use super::*;
use crate::types::*;

/// A tower in the game. Used to combat enemy waves.
#[derive(Debug, Clone)]
pub struct Tower {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<TowerInner>>,
}

#[derive(Debug, Clone)]
struct TowerInner {
    tower: Arc<Mutex<TowerBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct TowerBase {
    pub(crate) owner: Option<Player>,
    pub(crate) tile: Tile,
    pub(crate) job: TowerJob,
    pub(crate) health: i64,
    pub(crate) attacked: bool,
    pub(crate) cooldown: i64,
}

impl Tower {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<TowerInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Tower = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The player that built / owns this tower.
    pub fn owner(&self) -> Option<Player> {
        self.inner().tower.lock().unwrap().owner.clone()
    }

    /// The Tile this Tower is on.
    pub fn tile(&self) -> Tile {
        self.inner().tower.lock().unwrap().tile.clone()
    }

    /// What type of tower this is (it's job).
    pub fn job(&self) -> TowerJob {
        self.inner().tower.lock().unwrap().job.clone()
    }

    /// How much remaining health this tower has.
    pub fn health(&self) -> i64 {
        self.inner().tower.lock().unwrap().health.clone()
    }

    /// Whether this tower has attacked this turn or not.
    pub fn attacked(&self) -> bool {
        self.inner().tower.lock().unwrap().attacked.clone()
    }

    /// How many turns are left before it can fire again.
    pub fn cooldown(&self) -> i64 {
        self.inner().tower.lock().unwrap().cooldown.clone()
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

    /// Attacks an enemy unit on an tile within it's range.
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
