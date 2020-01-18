#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A machine in the game. Used to refine ore.
#[derive(Debug, Clone)]
pub struct Machine {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::GameObject>>,
}

impl Machine {
    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }

    /// The Tile this Machine is on.
    pub fn tile(&self) -> Tile {
        self.inner.lock().unwrap().as_machine()
            .tile.clone()
    }

    /// Tracks how many times this machine has been worked. (0 to refineTime).
    pub fn worked(&self) -> i64 {
        self.inner.lock().unwrap().as_machine()
            .worked.clone()
    }

    /// What type of ore the machine takes it. Also determines the type of material it outputs.
    /// (redium or blueium).
    pub fn ore_type(&self) -> Str {
        self.inner.lock().unwrap().as_machine()
            .ore_type.clone()
    }

    /// The number of times this machine needs to be worked to refine ore.
    pub fn refine_time(&self) -> i64 {
        self.inner.lock().unwrap().as_machine()
            .refine_time.clone()
    }

    /// The amount of ore that needs to be inputted into the machine for it to be worked.
    pub fn refine_input(&self) -> i64 {
        self.inner.lock().unwrap().as_machine()
            .refine_input.clone()
    }

    /// The amount of refined ore that is returned after the machine has been fully worked.
    pub fn refine_output(&self) -> i64 {
        self.inner.lock().unwrap().as_machine()
            .refine_output.clone()
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

impl inner::ObjectInner for Machine {
    fn from_game_object(game_obj: &Arc<Mutex<inner::GameObject>>, context: &Weak<Mutex<inner::Context>>) -> Option<Self> {
        let handle = game_obj.lock().unwrap();
        handle.try_as_machine()?;
        handle.try_as_game_object()?;
        Some(Machine {
            inner: Arc::clone(&game_obj),
            context: context.clone(),
        })
    }
}
impl Object for Machine {}
