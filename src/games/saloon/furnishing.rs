#![allow(dead_code, unused_imports)]

use std::sync::{Arc, Mutex, Weak};
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;

use super::*;
use crate::types::*;
use crate::error::Error;

/// An furnishing in the Saloon that must be pathed around, or destroyed.
#[derive(Debug, Clone)]
pub struct Furnishing {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<FurnishingInner>>,
}

#[derive(Debug, Clone)]
struct FurnishingInner {
    furnishing: Arc<Mutex<FurnishingBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct FurnishingBase {
    pub(crate) is_piano: bool,
    pub(crate) tile: Option<Tile>,
    pub(crate) health: i64,
    pub(crate) is_destroyed: bool,
    pub(crate) is_playing: bool,
}

impl Furnishing {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<FurnishingInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Furnishing = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// True if this Furnishing is a piano and can be played, False otherwise.
    pub fn is_piano(&self) -> bool {
        self.inner().furnishing.lock().unwrap().is_piano.clone()
    }

    /// The Tile that this Furnishing is located on.
    pub fn tile(&self) -> Option<Tile> {
        self.inner().furnishing.lock().unwrap().tile.clone()
    }

    /// How much health this Furnishing currently has.
    pub fn health(&self) -> i64 {
        self.inner().furnishing.lock().unwrap().health.clone()
    }

    /// If this Furnishing has been destroyed, and has been removed from the game.
    pub fn is_destroyed(&self) -> bool {
        self.inner().furnishing.lock().unwrap().is_destroyed.clone()
    }

    /// If this is a piano and a Cowboy is playing it this turn.
    pub fn is_playing(&self) -> bool {
        self.inner().furnishing.lock().unwrap().is_playing.clone()
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

    pub fn cast<T>(&self) -> T {
        self.context().get_obj(&self.id)
    }
}
