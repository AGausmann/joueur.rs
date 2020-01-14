#![allow(dead_code, unused_imports)]

use std::any::TypeId;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A person on the map that can move around and interact within the saloon.
#[derive(Debug, Clone)]
pub struct Cowboy {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<CowboyInner>>,
}

#[derive(Debug, Clone)]
struct CowboyInner {
    cowboy: Arc<Mutex<CowboyBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct CowboyBase {
    pub(crate) health: i64,
    pub(crate) owner: Player,
    pub(crate) is_dead: bool,
    pub(crate) job: Str,
    pub(crate) can_move: bool,
    pub(crate) tile: Option<Tile>,
    pub(crate) focus: i64,
    pub(crate) is_drunk: bool,
    pub(crate) drunk_direction: Str,
    pub(crate) tolerance: i64,
    pub(crate) turns_busy: i64,
}

impl Cowboy {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<CowboyInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Cowboy = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// How much health this Cowboy currently has.
    pub fn health(&self) -> i64 {
        self.inner().cowboy.lock().unwrap().health.clone()
    }

    /// The Player that owns and can control this Cowboy.
    pub fn owner(&self) -> Player {
        self.inner().cowboy.lock().unwrap().owner.clone()
    }

    /// If this Cowboy is dead and has been removed from the game.
    pub fn is_dead(&self) -> bool {
        self.inner().cowboy.lock().unwrap().is_dead.clone()
    }

    /// The job that this Cowboy does, and dictates how they fight and interact within the Saloon.
    pub fn job(&self) -> Str {
        self.inner().cowboy.lock().unwrap().job.clone()
    }

    /// If the Cowboy can be moved this turn via its owner.
    pub fn can_move(&self) -> bool {
        self.inner().cowboy.lock().unwrap().can_move.clone()
    }

    /// The Tile that this Cowboy is located on.
    pub fn tile(&self) -> Option<Tile> {
        self.inner().cowboy.lock().unwrap().tile.clone()
    }

    /// How much focus this Cowboy has. Different Jobs do different things with their Cowboy's
    /// focus.
    pub fn focus(&self) -> i64 {
        self.inner().cowboy.lock().unwrap().focus.clone()
    }

    /// If this Cowboy is drunk, and will automatically walk.
    pub fn is_drunk(&self) -> bool {
        self.inner().cowboy.lock().unwrap().is_drunk.clone()
    }

    /// The direction this Cowboy is moving while drunk. Will be 'North', 'East', 'South', or
    /// 'West' when drunk; or '' (empty string) when not drunk.
    pub fn drunk_direction(&self) -> Str {
        self.inner().cowboy.lock().unwrap().drunk_direction.clone()
    }

    /// How many times this unit has been drunk before taking their siesta and reseting this to 0.
    pub fn tolerance(&self) -> i64 {
        self.inner().cowboy.lock().unwrap().tolerance.clone()
    }

    /// How many turns this unit has remaining before it is no longer busy and can `act()` or
    /// `play()` again.
    pub fn turns_busy(&self) -> i64 {
        self.inner().cowboy.lock().unwrap().turns_busy.clone()
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

    /// Moves this Cowboy from its current Tile to an adjacent Tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile you want to move this Cowboy to.
    ///
    /// # Returns
    ///
    /// True if the move worked, false otherwise.
    pub fn move_(
        &self,
        tile: &Tile,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            tile: &'a Tile,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            tile,
            _a: PhantomData,
        };
        self.context().run(&self.id, "move", args)
    }

    /// Sits down and plays a piano.
    ///
    /// # Arguments
    ///
    /// - _piano_ - The Furnishing that is a piano you want to play.
    ///
    /// # Returns
    ///
    /// True if the play worked, false otherwise.
    pub fn play(
        &self,
        piano: &Furnishing,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            piano: &'a Furnishing,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            piano,
            _a: PhantomData,
        };
        self.context().run(&self.id, "play", args)
    }

    /// Does their job's action on a Tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile you want this Cowboy to act on.
    ///
    /// - _drunk_direction_ - The direction the bottle will cause drunk cowboys to be in, can be
    /// 'North', 'East', 'South', or 'West'.
    ///
    /// # Returns
    ///
    /// True if the act worked, false otherwise.
    pub fn act(
        &self,
        tile: &Tile,
        drunk_direction: &str,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            tile: &'a Tile,
            drunk_direction: &'a str,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            tile,
            drunk_direction,
            _a: PhantomData,
        };
        self.context().run(&self.id, "act", args)
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
            x if x == TypeId::of::<Cowboy>() => Some(T::shallow(self.context.clone(), self.id.clone())),
            x if x == TypeId::of::<GameObject>() => Some(T::shallow(self.context.clone(), self.id.clone())),
            _ => None,
        }
    }
}

impl ObjectInner for Cowboy {
    fn shallow(context: Weak<Context>, id: Str) -> Cowboy {
        Cowboy {
            context,
            id,
            inner: RefCell::new(None),
        }
    }
}

impl Object for Cowboy {}
