#![allow(dead_code, unused_imports)]

use std::sync::{Arc, Mutex, Weak};
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;

use super::*;
use crate::types::*;
use crate::error::Error;

/// A structure on a Tile.
#[derive(Debug, Clone)]
pub struct Structure {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<StructureInner>>,
}

#[derive(Debug, Clone)]
struct StructureInner {
    structure: Arc<Mutex<StructureBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct StructureBase {
    pub(crate) type_: Str,
    pub(crate) tile: Option<Tile>,
    pub(crate) owner: Option<Player>,
    pub(crate) materials: i64,
    pub(crate) effect_radius: i64,
}

impl Structure {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<StructureInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Structure = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The type of Structure this is ('shelter', 'monument', 'wall', 'road', 'neutral').
    pub fn type_(&self) -> Str {
        self.inner().structure.lock().unwrap().type_.clone()
    }

    /// The Tile this Structure is on.
    pub fn tile(&self) -> Option<Tile> {
        self.inner().structure.lock().unwrap().tile.clone()
    }

    /// The owner of this Structure if any, otherwise None.
    pub fn owner(&self) -> Option<Player> {
        self.inner().structure.lock().unwrap().owner.clone()
    }

    /// The number of materials in this Structure. Once this number reaches 0, this Structure is
    /// destroyed.
    pub fn materials(&self) -> i64 {
        self.inner().structure.lock().unwrap().materials.clone()
    }

    /// The range of this Structure's effect. For example, a radius of 1 means this Structure
    /// affects a 3x3 square centered on this Structure.
    pub fn effect_radius(&self) -> i64 {
        self.inner().structure.lock().unwrap().effect_radius.clone()
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
