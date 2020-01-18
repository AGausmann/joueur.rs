#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A location (node) connected to other Nests via Webs (edges) in the game that Spiders can
/// converge on, regardless of owner.
#[derive(Debug, Clone)]
pub struct Nest {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::GameObject>>,
}

impl Nest {
    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }

    /// Webs that connect to this Nest.
    pub fn webs(&self) -> List<Web> {
        self.inner.lock().unwrap().as_nest()
            .webs.clone()
    }

    /// All the Spiders currently located on this Nest.
    pub fn spiders(&self) -> List<Spider> {
        self.inner.lock().unwrap().as_nest()
            .spiders.clone()
    }

    /// The Player that 'controls' this Nest as they have the most Spiders on this nest.
    pub fn controlling_player(&self) -> Option<Player> {
        self.inner.lock().unwrap().as_nest()
            .controlling_player.clone()
    }

    /// The X coordinate of the Nest. Used for distance calculations.
    pub fn x(&self) -> i64 {
        self.inner.lock().unwrap().as_nest()
            .x.clone()
    }

    /// The Y coordinate of the Nest. Used for distance calculations.
    pub fn y(&self) -> i64 {
        self.inner.lock().unwrap().as_nest()
            .y.clone()
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

impl inner::ObjectInner for Nest {
    fn from_game_object(game_obj: &Arc<Mutex<inner::GameObject>>, context: &Weak<Mutex<inner::Context>>) -> Option<Self> {
        let handle = game_obj.lock().unwrap();
        handle.try_as_nest()?;
        handle.try_as_game_object()?;
        Some(Nest {
            inner: Arc::clone(&game_obj),
            context: context.clone(),
        })
    }
}
impl Object for Nest {}
