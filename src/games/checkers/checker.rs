#![allow(dead_code, unused_imports)]

use std::sync::{Arc, Mutex, Weak};
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;

use super::*;
use crate::types::*;
use crate::error::Error;

/// A checker on the game board.
#[derive(Debug, Clone)]
pub struct Checker {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<CheckerInner>>,
}

#[derive(Debug, Clone)]
struct CheckerInner {
    checker: Arc<Mutex<CheckerBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct CheckerBase {
    pub(crate) owner: Player,
    pub(crate) x: i64,
    pub(crate) y: i64,
    pub(crate) kinged: bool,
}

impl Checker {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<CheckerInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Checker = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The player that controls this Checker.
    pub fn owner(&self) -> Player {
        self.inner().checker.lock().unwrap().owner.clone()
    }

    /// The x coordinate of the checker.
    pub fn x(&self) -> i64 {
        self.inner().checker.lock().unwrap().x.clone()
    }

    /// The y coordinate of the checker.
    pub fn y(&self) -> i64 {
        self.inner().checker.lock().unwrap().y.clone()
    }

    /// If the checker has been kinged and can move backwards.
    pub fn kinged(&self) -> bool {
        self.inner().checker.lock().unwrap().kinged.clone()
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
        self.context().run(&self.id, "move", args)
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
        self.context().run(&self.id, "isMine", args)
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
