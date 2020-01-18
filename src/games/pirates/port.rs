#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A port on a Tile.
#[derive(Debug, Clone)]
pub struct Port {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::GameObject>>,
}

impl Port {
    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }

    /// The Tile this Port is on.
    pub fn tile(&self) -> Tile {
        self.inner.lock().unwrap().as_port()
            .tile.clone()
    }

    /// The owner of this Port, or None if owned by merchants.
    pub fn owner(&self) -> Option<Player> {
        self.inner.lock().unwrap().as_port()
            .owner.clone()
    }

    /// For players, how much more gold this Port can spend this turn. For merchants, how much gold
    /// this Port has accumulated (it will spawn a ship when the Port can afford one).
    pub fn gold(&self) -> i64 {
        self.inner.lock().unwrap().as_port()
            .gold.clone()
    }

    /// (Merchants only) How much gold was invested into this Port. Investment determines the
    /// strength and value of the next ship.
    pub fn investment(&self) -> i64 {
        self.inner.lock().unwrap().as_port()
            .investment.clone()
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

    /// Spawn a Unit on this port.
    ///
    /// # Arguments
    ///
    /// - _type__ - What type of Unit to create ('crew' or 'ship').
    ///
    /// # Returns
    ///
    /// True if Unit was created successfully, false otherwise.
    pub fn spawn(
        &self,
        type_: &str,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            type_: &'a str,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            type_,
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

    pub fn try_cast<T: Object>(&self) -> Option<T> {
        T::from_game_object(&self.inner, &self.context)
    }

    pub fn cast<T: Object>(&self) -> T {
        self.try_cast().unwrap()
    }
}

impl inner::ObjectInner for Port {
    fn from_game_object(game_obj: &Arc<Mutex<inner::GameObject>>, context: &Weak<Mutex<inner::Context>>) -> Option<Self> {
        let handle = game_obj.lock().unwrap();
        if handle.try_as_port().is_some() {
            Some(Port {
                inner: Arc::clone(&game_obj),
                context: context.clone(),
            })
        } else {
            None
        }
    }
}
impl Object for Port {}
