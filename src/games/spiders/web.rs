#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A connection (edge) to a Nest (node) in the game that Spiders can converge on (regardless of
/// owner). Spiders can travel in either direction on Webs.
#[derive(Debug, Clone)]
pub struct Web {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::GameObject>>,
}

impl Web {
    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }

    /// The first Nest this Web is connected to.
    pub fn nest_a(&self) -> Option<Nest> {
        self.inner.lock().unwrap().as_web()
            .nest_a.clone()
    }

    /// The second Nest this Web is connected to.
    pub fn nest_b(&self) -> Option<Nest> {
        self.inner.lock().unwrap().as_web()
            .nest_b.clone()
    }

    /// All the Spiderlings currently moving along this Web.
    pub fn spiderlings(&self) -> List<Spiderling> {
        self.inner.lock().unwrap().as_web()
            .spiderlings.clone()
    }

    /// How much weight this Web can take before snapping and destroying itself and all the Spiders
    /// on it.
    pub fn strength(&self) -> i64 {
        self.inner.lock().unwrap().as_web()
            .strength.clone()
    }

    /// How much weight this Web currently has on it, which is the sum of all its Spiderlings
    /// weight.
    pub fn load(&self) -> i64 {
        self.inner.lock().unwrap().as_web()
            .load.clone()
    }

    /// How long this Web is, i.e., the distance between its nestA and nestB.
    pub fn length(&self) -> f64 {
        self.inner.lock().unwrap().as_web()
            .length.clone()
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
}
