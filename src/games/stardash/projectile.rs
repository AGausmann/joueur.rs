#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// Tracks any projectiles moving through space.
#[derive(Debug, Clone)]
pub struct Projectile {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::GameObject>>,
}

impl Projectile {
    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }

    /// The Player that owns and can control this Projectile.
    pub fn owner(&self) -> Option<Player> {
        self.inner.lock().unwrap().as_projectile()
            .owner.clone()
    }

    /// The x value this projectile is on.
    pub fn x(&self) -> f64 {
        self.inner.lock().unwrap().as_projectile()
            .x.clone()
    }

    /// The y value this projectile is on.
    pub fn y(&self) -> f64 {
        self.inner.lock().unwrap().as_projectile()
            .y.clone()
    }

    /// The unit that is being attacked by this projectile.
    pub fn target(&self) -> Unit {
        self.inner.lock().unwrap().as_projectile()
            .target.clone()
    }

    /// The amount of remaining distance the projectile can move.
    pub fn fuel(&self) -> i64 {
        self.inner.lock().unwrap().as_projectile()
            .fuel.clone()
    }

    /// The remaining health of the projectile.
    pub fn energy(&self) -> i64 {
        self.inner.lock().unwrap().as_projectile()
            .energy.clone()
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
