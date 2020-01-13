#![allow(dead_code, unused_imports)]

use std::sync::{Arc, Mutex, Weak};
use std::cell::{RefCell, RefMut};

use super::*;
use crate::types::*;

/// A location (node) connected to other Nests via Webs (edges) in the game that Spiders can
/// converge on, regardless of owner.
#[derive(Debug, Clone)]
pub struct Nest {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<NestInner>>,
}

#[derive(Debug, Clone)]
struct NestInner {
    nest: Arc<Mutex<NestBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct NestBase {
    pub(crate) webs: List<Web>,
    pub(crate) spiders: List<Spider>,
    pub(crate) controlling_player: Option<Player>,
    pub(crate) x: i64,
    pub(crate) y: i64,
}

impl Nest {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<NestInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Nest = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// Webs that connect to this Nest.
    pub fn webs(&self) -> List<Web> {
        self.inner().nest.lock().unwrap().webs.clone()
    }

    /// All the Spiders currently located on this Nest.
    pub fn spiders(&self) -> List<Spider> {
        self.inner().nest.lock().unwrap().spiders.clone()
    }

    /// The Player that 'controls' this Nest as they have the most Spiders on this nest.
    pub fn controlling_player(&self) -> Option<Player> {
        self.inner().nest.lock().unwrap().controlling_player.clone()
    }

    /// The X coordinate of the Nest. Used for distance calculations.
    pub fn x(&self) -> i64 {
        self.inner().nest.lock().unwrap().x.clone()
    }

    /// The Y coordinate of the Nest. Used for distance calculations.
    pub fn y(&self) -> i64 {
        self.inner().nest.lock().unwrap().y.clone()
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
