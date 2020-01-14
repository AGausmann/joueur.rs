#![allow(dead_code, unused_imports)]

use std::any::TypeId;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A bottle thrown by a bartender at a Tile.
#[derive(Debug, Clone)]
pub struct Bottle {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<BottleInner>>,
}

#[derive(Debug, Clone)]
struct BottleInner {
    bottle: Arc<Mutex<BottleBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct BottleBase {
    pub(crate) tile: Option<Tile>,
    pub(crate) direction: Str,
    pub(crate) is_destroyed: bool,
    pub(crate) drunk_direction: Str,
}

impl Bottle {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<BottleInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Bottle = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The Tile this bottle is currently flying over.
    pub fn tile(&self) -> Option<Tile> {
        self.inner().bottle.lock().unwrap().tile.clone()
    }

    /// The Direction this Bottle is flying and will move to between turns, can be 'North', 'East',
    /// 'South', or 'West'.
    pub fn direction(&self) -> Str {
        self.inner().bottle.lock().unwrap().direction.clone()
    }

    /// True if this Bottle has impacted and has been destroyed (removed from the Game). False if
    /// still in the game flying through the saloon.
    pub fn is_destroyed(&self) -> bool {
        self.inner().bottle.lock().unwrap().is_destroyed.clone()
    }

    /// The direction any Cowboys hit by this will move, can be 'North', 'East', 'South', or
    /// 'West'.
    pub fn drunk_direction(&self) -> Str {
        self.inner().bottle.lock().unwrap().drunk_direction.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// A unique id for each instance of a GameObject or a sub class. Used for client and server
    /// communication. Should never change value after being set.
    pub fn id(&self) -> Str {
        self.inner().game_object.lock().unwrap().id.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// String representing the top level Class that this game object is an instance of. Used for
    /// reflection to create new instances on clients, but exposed for convenience should AIs want
    /// this data.
    pub fn game_object_name(&self) -> Str {
        self.inner().game_object.lock().unwrap().game_object_name.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// Any strings logged will be stored here. Intended for debugging.
    pub fn logs(&self) -> List<Str> {
        self.inner().game_object.lock().unwrap().logs.clone()
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

    pub fn try_cast<T: Object>(&self) -> Option<T> {
        self.context().try_get_obj(&self.id)
    }

    pub fn cast<T: Object>(&self) -> T {
        self.context().get_obj(&self.id)
    }

    pub(crate) fn try_upcast<T: Object>(&self) -> Option<T> {
        match TypeId::of::<T>() {
            x if x == TypeId::of::<Bottle>() => Some(T::shallow(self.context.clone(), self.id.clone())),
            x if x == TypeId::of::<GameObject>() => Some(T::shallow(self.context.clone(), self.id.clone())),
            _ => None,
        }
    }
}

impl ObjectInner for Bottle {
    fn shallow(context: Weak<Context>, id: Str) -> Bottle {
        Bottle {
            context,
            id,
            inner: RefCell::new(None),
        }
    }
}

impl Object for Bottle {}
