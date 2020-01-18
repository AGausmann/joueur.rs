#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A checker on the game board.
#[derive(Debug, Clone)]
pub struct Checker {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::GameObject>>,
}

impl Checker {
    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }

    /// The player that controls this Checker.
    pub fn owner(&self) -> Player {
        self.inner.lock().unwrap().as_checker()
            .owner.clone()
    }

    /// The x coordinate of the checker.
    pub fn x(&self) -> i64 {
        self.inner.lock().unwrap().as_checker()
            .x.clone()
    }

    /// The y coordinate of the checker.
    pub fn y(&self) -> i64 {
        self.inner.lock().unwrap().as_checker()
            .y.clone()
    }

    /// If the checker has been kinged and can move backwards.
    pub fn kinged(&self) -> bool {
        self.inner.lock().unwrap().as_checker()
            .kinged.clone()
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

    /// Moves the checker from its current location to the given (x, y).
    ///
    /// # Arguments
    ///
    /// - _x_ - The x coordinate to move to.
    ///
    /// - _y_ - The y coordinate to move to.
    ///
    /// # Returns
    ///
    /// Returns the same checker that moved if the move was successful. None otherwise.
    pub fn move_(
        &self,
        x: i64,
        y: i64,
    )
        -> Result<Option<Checker>, Error>
    {
        struct Args<'a> {
            x: i64,
            y: i64,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            x,
            y,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "move", args))
    }

    /// Returns if the checker is owned by your player or not.
    ///
    /// # Returns
    ///
    /// True if it is yours, false if it is not yours.
    pub fn is_mine(
        &self,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "isMine", args))
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
