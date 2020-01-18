#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// An eager young person that wants to join your gang, and will call in the veteran Cowboys you
/// need to win the brawl in the saloon.
#[derive(Debug, Clone)]
pub struct YoungGun {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::GameObject>>,
}

impl YoungGun {
    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }

    /// The Player that owns and can control this YoungGun.
    pub fn owner(&self) -> Player {
        self.inner.lock().unwrap().as_young_gun()
            .owner.clone()
    }

    /// The Tile this YoungGun is currently on.
    pub fn tile(&self) -> Tile {
        self.inner.lock().unwrap().as_young_gun()
            .tile.clone()
    }

    /// True if the YoungGun can call in a Cowboy, false otherwise.
    pub fn can_call_in(&self) -> bool {
        self.inner.lock().unwrap().as_young_gun()
            .can_call_in.clone()
    }

    /// The Tile that a Cowboy will be called in on if this YoungGun calls in a Cowboy.
    pub fn call_in_tile(&self) -> Tile {
        self.inner.lock().unwrap().as_young_gun()
            .call_in_tile.clone()
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

    /// Tells the YoungGun to call in a new Cowboy of the given job to the open Tile nearest to
    /// them.
    ///
    /// # Arguments
    ///
    /// - _job_ - The job you want the Cowboy being brought to have.
    ///
    /// # Returns
    ///
    /// The new Cowboy that was called in if valid. They will not be added to any `cowboys` lists
    /// until the turn ends. None otherwise.
    pub fn call_in(
        &self,
        job: &str,
    )
        -> Result<Option<Cowboy>, Error>
    {
        struct Args<'a> {
            job: &'a str,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            job,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "callIn", args))
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

impl inner::ObjectInner for YoungGun {
    fn from_game_object(game_obj: &Arc<Mutex<inner::GameObject>>, context: &Weak<Mutex<inner::Context>>) -> Option<Self> {
        let handle = game_obj.lock().unwrap();
        handle.try_as_young_gun()?;
        handle.try_as_game_object()?;
        Some(YoungGun {
            inner: Arc::clone(&game_obj),
            context: context.clone(),
        })
    }
}
impl Object for YoungGun {}
