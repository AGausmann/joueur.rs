#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A beaver in the game.
#[derive(Debug, Clone)]
pub struct Beaver {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::AnyGameObject>>,
}

impl Beaver {
    pub(crate) fn new(inner: Arc<Mutex<inner::AnyGameObject>>, context: Weak<Mutex<inner::Context>>) -> Beaver {
        Beaver { inner, context }
    }

    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }

    /// How many moves this Beaver has left this turn.
    pub fn moves(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_beaver()
            .moves.clone()
    }

    /// The Player that owns and can control this Beaver.
    pub fn owner(&self) -> Player {
        self.inner.lock().unwrap()
            .as_beaver()
            .owner.clone()
    }

    /// The number of actions remaining for the Beaver this turn.
    pub fn actions(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_beaver()
            .actions.clone()
    }

    /// The Tile this Beaver is on.
    pub fn tile(&self) -> Option<Tile> {
        self.inner.lock().unwrap()
            .as_beaver()
            .tile.clone()
    }

    /// How much health this Beaver has left.
    pub fn health(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_beaver()
            .health.clone()
    }

    /// Number of turns this Beaver is distracted for (0 means not distracted).
    pub fn turns_distracted(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_beaver()
            .turns_distracted.clone()
    }

    /// The amount of branches this Beaver is holding.
    pub fn branches(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_beaver()
            .branches.clone()
    }

    /// The amount of food this Beaver is holding.
    pub fn food(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_beaver()
            .food.clone()
    }

    /// The Job this Beaver was recruited to do.
    pub fn job(&self) -> Job {
        self.inner.lock().unwrap()
            .as_beaver()
            .job.clone()
    }

    /// True if the Beaver has finished being recruited and can do things, False otherwise.
    pub fn recruited(&self) -> bool {
        self.inner.lock().unwrap()
            .as_beaver()
            .recruited.clone()
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

    /// Moves this Beaver from its current Tile to an adjacent Tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile this Beaver should move to.
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
        self.with_context(|cx| cx.run(&self.id(), "move", args))
    }

    /// Harvests the branches or food from a Spawner on an adjacent Tile.
    ///
    /// # Arguments
    ///
    /// - _spawner_ - The Spawner you want to harvest. Must be on an adjacent Tile.
    ///
    /// # Returns
    ///
    /// True if successfully harvested, false otherwise.
    pub fn harvest(
        &self,
        spawner: &Spawner,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            spawner: &'a Spawner,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            spawner,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "harvest", args))
    }

    /// Attacks another adjacent beaver.
    ///
    /// # Arguments
    ///
    /// - _beaver_ - The Beaver to attack. Must be on an adjacent Tile.
    ///
    /// # Returns
    ///
    /// True if successfully attacked, false otherwise.
    pub fn attack(
        &self,
        beaver: &Beaver,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            beaver: &'a Beaver,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            beaver,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "attack", args))
    }

    /// Builds a lodge on the Beavers current Tile.
    ///
    /// # Returns
    ///
    /// True if successfully built a lodge, false otherwise.
    pub fn build_lodge(
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
        self.with_context(|cx| cx.run(&self.id(), "buildLodge", args))
    }

    /// Drops some of the given resource on the beaver's Tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile to drop branches/food on. Must be the same Tile that the Beaver is on,
    /// or an adjacent one.
    ///
    /// - _resource_ - The type of resource to drop ('branches' or 'food').
    ///
    /// - _amount_ - The amount of the resource to drop, numbers <= 0 will drop all the resource
    /// type.
    ///
    /// # Returns
    ///
    /// True if successfully dropped the resource, false otherwise.
    pub fn drop(
        &self,
        tile: &Tile,
        resource: &str,
        amount: i64,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            tile: &'a Tile,
            resource: &'a str,
            amount: i64,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            tile,
            resource,
            amount,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "drop", args))
    }

    /// Picks up some branches or food on the beaver's tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile to pickup branches/food from. Must be the same Tile that the Beaver is
    /// on, or an adjacent one.
    ///
    /// - _resource_ - The type of resource to pickup ('branches' or 'food').
    ///
    /// - _amount_ - The amount of the resource to drop, numbers <= 0 will pickup all of the
    /// resource type.
    ///
    /// # Returns
    ///
    /// True if successfully picked up a resource, false otherwise.
    pub fn pickup(
        &self,
        tile: &Tile,
        resource: &str,
        amount: i64,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            tile: &'a Tile,
            resource: &'a str,
            amount: i64,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            tile,
            resource,
            amount,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "pickup", args))
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

impl inner::ObjectInner for Beaver {
    fn from_game_object(game_obj: &Arc<Mutex<inner::AnyGameObject>>, context: &Weak<Mutex<inner::Context>>) -> Option<Self> {
        let handle = game_obj.lock().unwrap();
        if handle.try_as_beaver().is_some() {
            Some(Beaver {
                inner: Arc::clone(&game_obj),
                context: context.clone(),
            })
        } else {
            None
        }
    }
}
impl Object for Beaver {}
