#![allow(dead_code, unused_imports)]

use std::any::TypeId;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A player in this game. Every AI controls one player.
#[derive(Debug, Clone)]
pub struct Player {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<PlayerInner>>,
}

#[derive(Debug, Clone)]
struct PlayerInner {
    player: Arc<Mutex<PlayerBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct PlayerBase {
    pub(crate) name: Str,
    pub(crate) client_type: Str,
    pub(crate) won: bool,
    pub(crate) lost: bool,
    pub(crate) reason_won: Str,
    pub(crate) reason_lost: Str,
    pub(crate) time_remaining: f64,
    pub(crate) opponent: Player,
    pub(crate) color: Str,
}

impl Player {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<PlayerInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Player = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The name of the player.
    pub fn name(&self) -> Str {
        self.inner().player.lock().unwrap().name.clone()
    }

    /// What type of client this is, e.g. 'Python', 'JavaScript', or some other language. For
    /// potential data mining purposes.
    pub fn client_type(&self) -> Str {
        self.inner().player.lock().unwrap().client_type.clone()
    }

    /// If the player won the game or not.
    pub fn won(&self) -> bool {
        self.inner().player.lock().unwrap().won.clone()
    }

    /// If the player lost the game or not.
    pub fn lost(&self) -> bool {
        self.inner().player.lock().unwrap().lost.clone()
    }

    /// The reason why the player won the game.
    pub fn reason_won(&self) -> Str {
        self.inner().player.lock().unwrap().reason_won.clone()
    }

    /// The reason why the player lost the game.
    pub fn reason_lost(&self) -> Str {
        self.inner().player.lock().unwrap().reason_lost.clone()
    }

    /// The amount of time (in ns) remaining for this AI to send commands.
    pub fn time_remaining(&self) -> f64 {
        self.inner().player.lock().unwrap().time_remaining.clone()
    }

    /// This player's opponent in the game.
    pub fn opponent(&self) -> Player {
        self.inner().player.lock().unwrap().opponent.clone()
    }

    /// The color (side) of this player. Either 'white' or 'black', with the 'white' player having
    /// the first move.
    pub fn color(&self) -> Str {
        self.inner().player.lock().unwrap().color.clone()
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

    pub fn try_cast<T: Object>(&self) -> Option<T> {
        self.context().try_get_obj(&self.id)
    }

    pub fn cast<T: Object>(&self) -> T {
        self.context().get_obj(&self.id)
    }

    pub(crate) fn try_upcast<T: Object>(&self) -> Option<T> {
        match TypeId::of::<T>() {
            x if x == TypeId::of::<Player>() => Some(T::shallow(self.context.clone(), self.id.clone())),
            x if x == TypeId::of::<GameObject>() => Some(T::shallow(self.context.clone(), self.id.clone())),
            _ => None,
        }
    }
}

impl ObjectInner for Player {
    fn shallow(context: Weak<Context>, id: Str) -> Player {
        Player {
            context,
            id,
            inner: RefCell::new(None),
        }
    }
}

impl Object for Player {}
