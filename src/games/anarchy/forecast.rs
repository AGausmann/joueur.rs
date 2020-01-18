#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// The weather effect that will be applied at the end of a turn, which causes fires to spread.
#[derive(Debug, Clone)]
pub struct Forecast {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::AnyGameObject>>,
}

impl Forecast {
    pub(crate) fn new(inner: Arc<Mutex<inner::AnyGameObject>>, context: Weak<Mutex<inner::Context>>) -> Forecast {
        Forecast { inner, context }
    }

    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }

    /// The direction the wind will blow fires in. Can be 'north', 'east', 'south', or 'west'.
    pub fn direction(&self) -> Str {
        self.inner.lock().unwrap()
            .as_forecast()
            .direction.clone()
    }

    /// How much of a Building's fire that can be blown in the direction of this Forecast. Fire is
    /// duplicated (copied), not moved (transfered).
    pub fn intensity(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_forecast()
            .intensity.clone()
    }

    /// The Player that can use WeatherStations to control this Forecast when its the nextForecast.
    pub fn controlling_player(&self) -> Player {
        self.inner.lock().unwrap()
            .as_forecast()
            .controlling_player.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// A unique id for each instance of a GameObject or a sub class. Used for client and server
    /// communication. Should never change value after being set.
    pub fn id(&self) -> Str {
        self.inner.lock().unwrap()
            .as_game_object()
            .id.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// String representing the top level Class that this game object is an instance of. Used for
    /// reflection to create new instances on clients, but exposed for convenience should AIs want
    /// this data.
    pub fn game_object_name(&self) -> Str {
        self.inner.lock().unwrap()
            .as_game_object()
            .game_object_name.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// Any strings logged will be stored here. Intended for debugging.
    pub fn logs(&self) -> List<Str> {
        self.inner.lock().unwrap()
            .as_game_object()
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

impl inner::ObjectInner for Forecast {
    fn from_game_object(game_obj: &Arc<Mutex<inner::AnyGameObject>>, context: &Weak<Mutex<inner::Context>>) -> Option<Self> {
        let handle = game_obj.lock().unwrap();
        if handle.try_as_forecast().is_some() {
            Some(Forecast {
                inner: Arc::clone(&game_obj),
                context: context.clone(),
            })
        } else {
            None
        }
    }
}
impl Object for Forecast {}
