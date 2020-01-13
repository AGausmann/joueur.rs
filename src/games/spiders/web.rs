#![allow(dead_code, unused_imports)]

use std::sync::{Arc, Mutex, Weak};
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;

use super::*;
use crate::types::*;
use crate::error::Error;

/// A connection (edge) to a Nest (node) in the game that Spiders can converge on (regardless of
/// owner). Spiders can travel in either direction on Webs.
#[derive(Debug, Clone)]
pub struct Web {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<WebInner>>,
}

#[derive(Debug, Clone)]
struct WebInner {
    web: Arc<Mutex<WebBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct WebBase {
    pub(crate) nest_a: Option<Nest>,
    pub(crate) nest_b: Option<Nest>,
    pub(crate) spiderlings: List<Spiderling>,
    pub(crate) strength: i64,
    pub(crate) load: i64,
    pub(crate) length: f64,
}

impl Web {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<WebInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Web = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The first Nest this Web is connected to.
    pub fn nest_a(&self) -> Option<Nest> {
        self.inner().web.lock().unwrap().nest_a.clone()
    }

    /// The second Nest this Web is connected to.
    pub fn nest_b(&self) -> Option<Nest> {
        self.inner().web.lock().unwrap().nest_b.clone()
    }

    /// All the Spiderlings currently moving along this Web.
    pub fn spiderlings(&self) -> List<Spiderling> {
        self.inner().web.lock().unwrap().spiderlings.clone()
    }

    /// How much weight this Web can take before snapping and destroying itself and all the Spiders
    /// on it.
    pub fn strength(&self) -> i64 {
        self.inner().web.lock().unwrap().strength.clone()
    }

    /// How much weight this Web currently has on it, which is the sum of all its Spiderlings
    /// weight.
    pub fn load(&self) -> i64 {
        self.inner().web.lock().unwrap().load.clone()
    }

    /// How long this Web is, i.e., the distance between its nestA and nestB.
    pub fn length(&self) -> f64 {
        self.inner().web.lock().unwrap().length.clone()
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

    pub fn cast<T>(&self) -> Option<T> {
        self.context().get_obj(&self.id)
    }
}
