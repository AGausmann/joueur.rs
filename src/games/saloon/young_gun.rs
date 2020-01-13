#![allow(dead_code, unused_imports)]

use std::sync::{Arc, Mutex, Weak};
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;

use super::*;
use crate::types::*;
use crate::error::Error;

/// An eager young person that wants to join your gang, and will call in the veteran Cowboys you
/// need to win the brawl in the saloon.
#[derive(Debug, Clone)]
pub struct YoungGun {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<YoungGunInner>>,
}

#[derive(Debug, Clone)]
struct YoungGunInner {
    young_gun: Arc<Mutex<YoungGunBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct YoungGunBase {
    pub(crate) owner: Player,
    pub(crate) tile: Tile,
    pub(crate) can_call_in: bool,
    pub(crate) call_in_tile: Tile,
}

impl YoungGun {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<YoungGunInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: YoungGun = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The Player that owns and can control this YoungGun.
    pub fn owner(&self) -> Player {
        self.inner().young_gun.lock().unwrap().owner.clone()
    }

    /// The Tile this YoungGun is currently on.
    pub fn tile(&self) -> Tile {
        self.inner().young_gun.lock().unwrap().tile.clone()
    }

    /// True if the YoungGun can call in a Cowboy, false otherwise.
    pub fn can_call_in(&self) -> bool {
        self.inner().young_gun.lock().unwrap().can_call_in.clone()
    }

    /// The Tile that a Cowboy will be called in on if this YoungGun calls in a Cowboy.
    pub fn call_in_tile(&self) -> Tile {
        self.inner().young_gun.lock().unwrap().call_in_tile.clone()
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

    /// Tells the YoungGun to call in a new Cowboy of the given job to the open Tile nearest to
    /// them.
    ///
    /// # Arguments
    ///
    /// - _job_ - The job you want the Cowboy being brought to have.
    ///
    /// # Returns
    ///
    /// The new Cowboy that was called in if valid. They will not be added to any `cowboys` lists
    /// until the turn ends. None otherwise.
    pub fn call_in(
        &self,
        job: &str,
    )
        -> Result<Option<Cowboy>, Error>
    {
        struct Args<'a> {
            job: &'a str,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            job,
            _a: PhantomData,
        };
        self.context().run(&self.id, "callIn", args)
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
        message: &str,
    )
        -> Result<(), Error>
    {
        struct Args<'a> {
            message: &'a str,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            message,
            _a: PhantomData,
        };
        self.context().run(&self.id, "log", args)
    }

    pub fn try_cast<T>(&self) -> Option<T> {
        self.context().try_get_obj(&self.id)
    }

    pub fn cast<T>(&self) -> Option<T> {
        self.context().get_obj(&self.id)
    }
}
