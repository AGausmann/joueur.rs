#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// Information about a Unit's job.
#[derive(Debug, Clone)]
pub struct Job {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::GameObject>>,
}

impl Job {
    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }

    /// The Job title.
    pub fn title(&self) -> Str {
        self.inner.lock().unwrap().as_job()
            .title.clone()
    }

    /// The number of moves this Job can make per turn.
    pub fn moves(&self) -> i64 {
        self.inner.lock().unwrap().as_job()
            .moves.clone()
    }

    /// The amount of energy this Job normally uses to perform its actions.
    pub fn action_cost(&self) -> f64 {
        self.inner.lock().unwrap().as_job()
            .action_cost.clone()
    }

    /// The amount of energy normally regenerated when resting at a shelter.
    pub fn regen_rate(&self) -> f64 {
        self.inner.lock().unwrap().as_job()
            .regen_rate.clone()
    }

    /// How many combined resources a Unit with this Job can hold at once.
    pub fn carry_limit(&self) -> i64 {
        self.inner.lock().unwrap().as_job()
            .carry_limit.clone()
    }

    /// The amount of food per turn this Unit consumes. If there isn't enough food for every Unit,
    /// all Units become starved and do not consume food.
    pub fn upkeep(&self) -> i64 {
        self.inner.lock().unwrap().as_job()
            .upkeep.clone()
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

impl inner::ObjectInner for Job {
    fn from_game_object(game_obj: &Arc<Mutex<inner::GameObject>>, context: &Weak<Mutex<inner::Context>>) -> Option<Self> {
        let handle = game_obj.lock().unwrap();
        handle.try_as_job()?;
        handle.try_as_game_object()?;
        Some(Job {
            inner: Arc::clone(&game_obj),
            context: context.clone(),
        })
    }
}
impl Object for Job {}
