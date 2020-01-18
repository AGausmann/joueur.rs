#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A Spider spawned by the BroodMother.
#[derive(Debug, Clone)]
pub struct Spiderling {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::GameObject>>,
}

impl Spiderling {
    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }

    /// When empty string this Spiderling is not busy, and can act. Otherwise a string representing
    /// what it is busy with, e.g. 'Moving', 'Attacking'.
    pub fn busy(&self) -> Str {
        self.inner.lock().unwrap().as_spiderling()
            .busy.clone()
    }

    /// How much work needs to be done for this Spiderling to finish being busy. See docs for the
    /// Work forumla.
    pub fn work_remaining(&self) -> f64 {
        self.inner.lock().unwrap().as_spiderling()
            .work_remaining.clone()
    }

    /// The number of Spiderlings busy with the same work this Spiderling is doing, speeding up the
    /// task.
    pub fn number_of_coworkers(&self) -> i64 {
        self.inner.lock().unwrap().as_spiderling()
            .number_of_coworkers.clone()
    }

    /// The Web this Spiderling is using to move. None if it is not moving.
    pub fn moving_on_web(&self) -> Option<Web> {
        self.inner.lock().unwrap().as_spiderling()
            .moving_on_web.clone()
    }

    /// The Nest this Spiderling is moving to. None if it is not moving.
    pub fn moving_to_nest(&self) -> Option<Nest> {
        self.inner.lock().unwrap().as_spiderling()
            .moving_to_nest.clone()
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

    /// Starts moving the Spiderling across a Web to another Nest.
    ///
    /// # Arguments
    ///
    /// - _web_ - The Web you want to move across to the other Nest.
    ///
    /// # Returns
    ///
    /// True if the move was successful, false otherwise.
    pub fn move_(
        &self,
        web: &Web,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            web: &'a Web,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            web,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "move", args))
    }

    /// Attacks another Spiderling
    ///
    /// # Arguments
    ///
    /// - _spiderling_ - The Spiderling to attack.
    ///
    /// # Returns
    ///
    /// True if the attack was successful, false otherwise.
    pub fn attack(
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
        self.with_context(|cx| cx.run(&self.id(), "attack", args))
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

impl inner::ObjectInner for Spiderling {
    fn from_game_object(game_obj: &Arc<Mutex<inner::GameObject>>, context: &Weak<Mutex<inner::Context>>) -> Option<Self> {
        let handle = game_obj.lock().unwrap();
        handle.try_as_spiderling()?;
        handle.try_as_spider()?;
        handle.try_as_game_object()?;
        Some(Spiderling {
            inner: Arc::clone(&game_obj),
            context: context.clone(),
        })
    }
}
impl Object for Spiderling {}
