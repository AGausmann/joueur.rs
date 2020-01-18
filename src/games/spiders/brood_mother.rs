#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// The Spider Queen. She alone can spawn Spiderlings for each Player, and if she dies the owner
/// loses.
#[derive(Debug, Clone)]
pub struct BroodMother {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::GameObject>>,
}

impl BroodMother {
    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }

    /// How much health this BroodMother has left. When it reaches 0, she dies and her owner loses.
    pub fn health(&self) -> i64 {
        self.inner.lock().unwrap().as_brood_mother()
            .health.clone()
    }

    /// How many eggs the BroodMother has to spawn Spiderlings this turn.
    pub fn eggs(&self) -> i64 {
        self.inner.lock().unwrap().as_brood_mother()
            .eggs.clone()
    }

    /// _Inherited from [`Spider`]_
    ///
    /// The Player that owns this Spider, and can command it.
    pub fn owner(&self) -> Player {
        self.inner.lock().unwrap().as_spider()
            .owner.clone()
    }

    /// _Inherited from [`Spider`]_
    ///
    /// The Nest that this Spider is currently on. None when moving on a Web.
    pub fn nest(&self) -> Option<Nest> {
        self.inner.lock().unwrap().as_spider()
            .nest.clone()
    }

    /// _Inherited from [`Spider`]_
    ///
    /// If this Spider is dead and has been removed from the game.
    pub fn is_dead(&self) -> bool {
        self.inner.lock().unwrap().as_spider()
            .is_dead.clone()
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

    /// Consumes a Spiderling of the same owner to regain some eggs to spawn more Spiderlings.
    ///
    /// # Arguments
    ///
    /// - _spiderling_ - The Spiderling to consume. It must be on the same Nest as this
    /// BroodMother.
    ///
    /// # Returns
    ///
    /// True if the Spiderling was consumed. False otherwise.
    pub fn consume(
        &self,
        spiderling: &Spiderling,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            spiderling: &'a Spiderling,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            spiderling,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "consume", args))
    }

    /// Spawns a new Spiderling on the same Nest as this BroodMother, consuming an egg.
    ///
    /// # Arguments
    ///
    /// - _spiderling_type_ - The string name of the Spiderling class you want to Spawn. Must be
    /// 'Spitter', 'Weaver', or 'Cutter'.
    ///
    /// # Returns
    ///
    /// The newly spwaned Spiderling if successful. None otherwise.
    pub fn spawn(
        &self,
        spiderling_type: &str,
    )
        -> Result<Option<Spiderling>, Error>
    {
        struct Args<'a> {
            spiderling_type: &'a str,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            spiderling_type,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "spawn", args))
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
