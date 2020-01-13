#![allow(dead_code, unused_imports)]

use std::sync::{Arc, Mutex, Weak};
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;

use super::*;
use crate::types::*;
use crate::error::Error;

/// The weather effect that will be applied at the end of a turn, which causes fires to spread.
#[derive(Debug, Clone)]
pub struct Forecast {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<ForecastInner>>,
}

#[derive(Debug, Clone)]
struct ForecastInner {
    forecast: Arc<Mutex<ForecastBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct ForecastBase {
    pub(crate) direction: Str,
    pub(crate) intensity: i64,
    pub(crate) controlling_player: Player,
}

impl Forecast {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<ForecastInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Forecast = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The direction the wind will blow fires in. Can be 'north', 'east', 'south', or 'west'.
    pub fn direction(&self) -> Str {
        self.inner().forecast.lock().unwrap().direction.clone()
    }

    /// How much of a Building's fire that can be blown in the direction of this Forecast. Fire is
    /// duplicated (copied), not moved (transfered).
    pub fn intensity(&self) -> i64 {
        self.inner().forecast.lock().unwrap().intensity.clone()
    }

    /// The Player that can use WeatherStations to control this Forecast when its the nextForecast.
    pub fn controlling_player(&self) -> Player {
        self.inner().forecast.lock().unwrap().controlling_player.clone()
    }

    /// _Inherited from GameObject_
    ///
    /// A unique id for each instance of a GameObject or a sub class. Used for client and server
    /// communication. Should never change value after being set.
    pub fn id(&self) -> Str {
        self.inner().game_object.lock().unwrap().id.clone()
    }

    /// _Inherited from GameObject_
    ///
    /// String representing the top level Class that this game object is an instance of. Used for
    /// reflection to create new instances on clients, but exposed for convenience should AIs want
    /// this data.
    pub fn game_object_name(&self) -> Str {
        self.inner().game_object.lock().unwrap().game_object_name.clone()
    }

    /// _Inherited from GameObject_
    ///
    /// Any strings logged will be stored here. Intended for debugging.
    pub fn logs(&self) -> List<Str> {
        self.inner().game_object.lock().unwrap().logs.clone()
    }

    /// _Inherited from GameObject_
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
        self.context().run(&self.id, "log", args)
    }

    pub fn try_cast<T>(&self) -> Option<T> {
        self.context().try_get_obj(&self.id)
    }

    pub fn cast<T>(&self) -> T {
        self.context().get_obj(&self.id)
    }
}
