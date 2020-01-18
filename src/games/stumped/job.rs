#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// Information about a beaver's job.
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

    /// The amount of starting health this Job has.
    pub fn health(&self) -> i64 {
        self.inner.lock().unwrap().as_job()
            .health.clone()
    }

    /// The number of moves this Job can make per turn.
    pub fn moves(&self) -> i64 {
        self.inner.lock().unwrap().as_job()
            .moves.clone()
    }

    /// The number of actions this Job can make per turn.
    pub fn actions(&self) -> i64 {
        self.inner.lock().unwrap().as_job()
            .actions.clone()
    }

    /// The amount of damage this Job does per attack.
    pub fn damage(&self) -> i64 {
        self.inner.lock().unwrap().as_job()
            .damage.clone()
    }

    /// Scalar for how many branches this Job harvests at once.
    pub fn chopping(&self) -> i64 {
        self.inner.lock().unwrap().as_job()
            .chopping.clone()
    }

    /// Scalar for how much food this Job harvests at once.
    pub fn munching(&self) -> i64 {
        self.inner.lock().unwrap().as_job()
            .munching.clone()
    }

    /// How many turns a beaver attacked by this Job is distracted by.
    pub fn distraction_power(&self) -> i64 {
        self.inner.lock().unwrap().as_job()
            .distraction_power.clone()
    }

    /// How many combined resources a beaver with this Job can hold at once.
    pub fn carry_limit(&self) -> i64 {
        self.inner.lock().unwrap().as_job()
            .carry_limit.clone()
    }

    /// How much food this Job costs to recruit.
    pub fn cost(&self) -> i64 {
        self.inner.lock().unwrap().as_job()
            .cost.clone()
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

    /// Recruits a Beaver of this Job to a lodge
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile that is a lodge owned by you that you wish to spawn the Beaver of this
    /// Job on.
    ///
    /// # Returns
    ///
    /// The recruited Beaver if successful, None otherwise.
    pub fn recruit(
        &self,
        tile: &Tile,
    )
        -> Result<Option<Beaver>, Error>
    {
        struct Args<'a> {
            tile: &'a Tile,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            tile,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "recruit", args))
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
