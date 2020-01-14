#![allow(dead_code, unused_imports)]

use std::any::TypeId;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A celestial body located within the game.
#[derive(Debug, Clone)]
pub struct Body {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<BodyInner>>,
}

#[derive(Debug, Clone)]
struct BodyInner {
    body: Arc<Mutex<BodyBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct BodyBase {
    pub(crate) owner: Option<Player>,
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) radius: f64,
    pub(crate) body_type: Str,
    pub(crate) material_type: Str,
    pub(crate) amount: i64,
}

impl Body {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<BodyInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Body = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The Player that owns and can control this Body.
    pub fn owner(&self) -> Option<Player> {
        self.inner().body.lock().unwrap().owner.clone()
    }

    /// The x value this celestial body is on.
    pub fn x(&self) -> f64 {
        self.inner().body.lock().unwrap().x.clone()
    }

    /// The y value this celestial body is on.
    pub fn y(&self) -> f64 {
        self.inner().body.lock().unwrap().y.clone()
    }

    /// The radius of the circle that this body takes up.
    pub fn radius(&self) -> f64 {
        self.inner().body.lock().unwrap().radius.clone()
    }

    /// The type of celestial body it is. Either 'planet', 'asteroid', or 'sun'.
    pub fn body_type(&self) -> Str {
        self.inner().body.lock().unwrap().body_type.clone()
    }

    /// The type of material the celestial body has. Either 'none', 'genarium', 'rarium',
    /// 'legendarium', or 'mythicite'.
    pub fn material_type(&self) -> Str {
        self.inner().body.lock().unwrap().material_type.clone()
    }

    /// The amount of material the object has, or energy if it is a planet.
    pub fn amount(&self) -> i64 {
        self.inner().body.lock().unwrap().amount.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// A unique id for each instance of a GameObject or a sub class. Used for client and server
    /// communication. Should never change value after being set.
    pub fn id(&self) -> Str {
        self.inner().game_object.lock().unwrap().id.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// String representing the top level Class that this game object is an instance of. Used for
    /// reflection to create new instances on clients, but exposed for convenience should AIs want
    /// this data.
    pub fn game_object_name(&self) -> Str {
        self.inner().game_object.lock().unwrap().game_object_name.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// Any strings logged will be stored here. Intended for debugging.
    pub fn logs(&self) -> List<Str> {
        self.inner().game_object.lock().unwrap().logs.clone()
    }

    /// Spawn a unit on some value of this celestial body.
    ///
    /// # Arguments
    ///
    /// - _x_ - The x value of the spawned unit.
    ///
    /// - _y_ - The y value of the spawned unit.
    ///
    /// - _title_ - The job title of the unit being spawned.
    ///
    /// # Returns
    ///
    /// True if successfully taken, false otherwise.
    pub fn spawn(
        &self,
        x: f64,
        y: f64,
        title: &str,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            x: f64,
            y: f64,
            title: &'a str,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            x,
            y,
            title,
            _a: PhantomData,
        };
        self.context().run(&self.id, "spawn", args)
    }

    /// The x value of this body a number of turns from now. (0-how many you want).
    ///
    /// # Arguments
    ///
    /// - _num_ - The number of turns in the future you wish to check.
    ///
    /// # Returns
    ///
    /// The x position of the body the input number of turns in the future.
    pub fn next_x(
        &self,
        num: i64,
    )
        -> Result<i64, Error>
    {
        struct Args<'a> {
            num: i64,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            num,
            _a: PhantomData,
        };
        self.context().run(&self.id, "nextX", args)
    }

    /// The x value of this body a number of turns from now. (0-how many you want).
    ///
    /// # Arguments
    ///
    /// - _num_ - The number of turns in the future you wish to check.
    ///
    /// # Returns
    ///
    /// The x position of the body the input number of turns in the future.
    pub fn next_y(
        &self,
        num: i64,
    )
        -> Result<i64, Error>
    {
        struct Args<'a> {
            num: i64,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            num,
            _a: PhantomData,
        };
        self.context().run(&self.id, "nextY", args)
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
        self.context().run(&self.id, "log", args)
    }

    pub fn try_cast<T: Object>(&self) -> Option<T> {
        self.context().try_get_obj(&self.id)
    }

    pub fn cast<T: Object>(&self) -> T {
        self.context().get_obj(&self.id)
    }

    pub(crate) fn try_upcast<T: Object>(&self) -> Option<T> {
        match TypeId::of::<T>() {
            x if x == TypeId::of::<Body>() => Some(T::shallow(self.context.clone(), self.id.clone())),
            x if x == TypeId::of::<GameObject>() => Some(T::shallow(self.context.clone(), self.id.clone())),
            _ => None,
        }
    }
}

impl ObjectInner for Body {
    fn shallow(context: Weak<Context>, id: Str) -> Body {
        Body {
            context,
            id,
            inner: RefCell::new(None),
        }
    }
}

impl Object for Body {}
