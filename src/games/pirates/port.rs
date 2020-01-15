#![allow(dead_code, unused_imports)]

use std::any::TypeId;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A port on a Tile.
#[derive(Debug, Clone)]
pub struct Port {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<PortInner>>,
}

#[derive(Debug, Clone)]
struct PortInner {
    port: Arc<Mutex<PortBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct PortBase {
    pub(crate) tile: Tile,
    pub(crate) owner: Option<Player>,
    pub(crate) gold: i64,
    pub(crate) investment: i64,
}

impl Port {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<PortInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Port = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The Tile this Port is on.
    pub fn tile(&self) -> Tile {
        self.inner().port.lock().unwrap().tile.clone()
    }

    /// The owner of this Port, or None if owned by merchants.
    pub fn owner(&self) -> Option<Player> {
        self.inner().port.lock().unwrap().owner.clone()
    }

    /// For players, how much more gold this Port can spend this turn. For merchants, how much gold
    /// this Port has accumulated (it will spawn a ship when the Port can afford one).
    pub fn gold(&self) -> i64 {
        self.inner().port.lock().unwrap().gold.clone()
    }

    /// (Merchants only) How much gold was invested into this Port. Investment determines the
    /// strength and value of the next ship.
    pub fn investment(&self) -> i64 {
        self.inner().port.lock().unwrap().investment.clone()
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
        self.context().run(&self.id, "spawn", args)
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
}

impl ObjectInner for Port {
    fn to_bases(&self) -> Bases {
        let inner = self.inner();
        Bases {
            context: Some(self.context.clone()),
            id: Some(self.id.clone()),
            port: Some(Arc::clone(&inner.port)),
            game_object: Some(Arc::clone(&inner.game_object)),
            ..Default::default()
        }
    }

    fn from_bases(bases: Bases) -> Option<Self> {
        let inner = PortInner {
            port: bases.port?,
            game_object: bases.game_object?,
        };

        Some(Port {
            context: bases.context?,
            id: bases.id?,
            inner: RefCell::new(Some(inner)),
        })
    }
}

impl Object for Port {}
