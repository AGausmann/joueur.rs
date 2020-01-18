#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A structure on a Tile.
#[derive(Debug, Clone)]
pub struct Structure {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::GameObject>>,
}

impl Structure {
    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }

    /// The type of Structure this is ('shelter', 'monument', 'wall', 'road', 'neutral').
    pub fn type_(&self) -> Str {
        self.inner.lock().unwrap().as_structure()
            .type_.clone()
    }

    /// The Tile this Structure is on.
    pub fn tile(&self) -> Option<Tile> {
        self.inner.lock().unwrap().as_structure()
            .tile.clone()
    }

    /// The owner of this Structure if any, otherwise None.
    pub fn owner(&self) -> Option<Player> {
        self.inner.lock().unwrap().as_structure()
            .owner.clone()
    }

    /// The number of materials in this Structure. Once this number reaches 0, this Structure is
    /// destroyed.
    pub fn materials(&self) -> i64 {
        self.inner.lock().unwrap().as_structure()
            .materials.clone()
    }

    /// The range of this Structure's effect. For example, a radius of 1 means this Structure
    /// affects a 3x3 square centered on this Structure.
    pub fn effect_radius(&self) -> i64 {
        self.inner.lock().unwrap().as_structure()
            .effect_radius.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// A unique id for each instance of a GameObject or a sub class. Used for client and server
    /// communication. Should never change value after being set.
    pub fn id(&self) -> Str {
        self.inner.lock().unwrap().as_game_object()
            .id.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// String representing the top level Class that this game object is an instance of. Used for
    /// reflection to create new instances on clients, but exposed for convenience should AIs want
    /// this data.
    pub fn game_object_name(&self) -> Str {
        self.inner.lock().unwrap().as_game_object()
            .game_object_name.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// Any strings logged will be stored here. Intended for debugging.
    pub fn logs(&self) -> List<Str> {
        self.inner.lock().unwrap().as_game_object()
            .logs.clone()
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
        self.with_context(|cx| cx.run(&self.id(), "log", args))
    }

    pub fn try_cast<T: Object>(&self) -> Option<T> {
        T::from_game_object(&self.inner, &self.context)
    }

    pub fn cast<T: Object>(&self) -> T {
        self.try_cast().unwrap()
    }
}

impl inner::ObjectInner for Structure {
    fn from_game_object(game_obj: &Arc<Mutex<inner::GameObject>>, context: &Weak<Mutex<inner::Context>>) -> Option<Self> {
        let handle = game_obj.lock().unwrap();
        handle.try_as_structure()?;
        handle.try_as_game_object()?;
        Some(Structure {
            inner: Arc::clone(&game_obj),
            context: context.clone(),
        })
    }
}
impl Object for Structure {}
