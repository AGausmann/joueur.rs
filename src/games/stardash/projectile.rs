#![allow(dead_code, unused_imports)]

use std::sync::{Arc, Mutex, Weak};
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;

use super::*;
use crate::types::*;
use crate::error::Error;

/// Tracks any projectiles moving through space.
#[derive(Debug, Clone)]
pub struct Projectile {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<ProjectileInner>>,
}

#[derive(Debug, Clone)]
struct ProjectileInner {
    projectile: Arc<Mutex<ProjectileBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct ProjectileBase {
    pub(crate) owner: Option<Player>,
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) target: Unit,
    pub(crate) fuel: i64,
    pub(crate) energy: i64,
}

impl Projectile {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<ProjectileInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Projectile = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The Player that owns and can control this Projectile.
    pub fn owner(&self) -> Option<Player> {
        self.inner().projectile.lock().unwrap().owner.clone()
    }

    /// The x value this projectile is on.
    pub fn x(&self) -> f64 {
        self.inner().projectile.lock().unwrap().x.clone()
    }

    /// The y value this projectile is on.
    pub fn y(&self) -> f64 {
        self.inner().projectile.lock().unwrap().y.clone()
    }

    /// The unit that is being attacked by this projectile.
    pub fn target(&self) -> Unit {
        self.inner().projectile.lock().unwrap().target.clone()
    }

    /// The amount of remaining distance the projectile can move.
    pub fn fuel(&self) -> i64 {
        self.inner().projectile.lock().unwrap().fuel.clone()
    }

    /// The remaining health of the projectile.
    pub fn energy(&self) -> i64 {
        self.inner().projectile.lock().unwrap().energy.clone()
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

    pub fn cast<T>(&self) -> Option<T> {
        self.context().get_obj(&self.id)
    }
}
