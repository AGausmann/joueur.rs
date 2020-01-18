#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A unit in the game.
#[derive(Debug, Clone)]
pub struct Unit {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::AnyGameObject>>,
}

impl Unit {
    pub(crate) fn new(inner: Arc<Mutex<inner::AnyGameObject>>, context: Weak<Mutex<inner::Context>>) -> Unit {
        Unit { inner, context }
    }

    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }

    /// The Player that owns and can control this Unit, or None if the Unit is neutral.
    pub fn owner(&self) -> Option<Player> {
        self.inner.lock().unwrap()
            .as_unit()
            .owner.clone()
    }

    /// The Tile this Unit is on.
    pub fn tile(&self) -> Option<Tile> {
        self.inner.lock().unwrap()
            .as_unit()
            .tile.clone()
    }

    /// The Job this Unit was recruited to do.
    pub fn job(&self) -> Job {
        self.inner.lock().unwrap()
            .as_unit()
            .job.clone()
    }

    /// How many moves this Unit has left this turn.
    pub fn moves(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_unit()
            .moves.clone()
    }

    /// The amount of energy this Unit has (from 0.0 to 100.0).
    pub fn energy(&self) -> f64 {
        self.inner.lock().unwrap()
            .as_unit()
            .energy.clone()
    }

    /// The Units in the same squad as this Unit. Units in the same squad attack and defend
    /// together.
    pub fn squad(&self) -> List<Unit> {
        self.inner.lock().unwrap()
            .as_unit()
            .squad.clone()
    }

    /// Whether this Unit has performed its action this turn.
    pub fn acted(&self) -> bool {
        self.inner.lock().unwrap()
            .as_unit()
            .acted.clone()
    }

    /// The amount of food this Unit is holding.
    pub fn food(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_unit()
            .food.clone()
    }

    /// The amount of materials this Unit is holding.
    pub fn materials(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_unit()
            .materials.clone()
    }

    /// Whether this Unit is starving. Starving Units regenerate energy at half the rate they
    /// normally would while resting.
    pub fn starving(&self) -> bool {
        self.inner.lock().unwrap()
            .as_unit()
            .starving.clone()
    }

    /// The number of turns before this Unit dies. This only applies to neutral fresh humans
    /// created from combat. Otherwise, 0.
    pub fn turns_to_die(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_unit()
            .turns_to_die.clone()
    }

    /// The tile this Unit is moving to. This only applies to neutral fresh humans spawned on the
    /// road. Otherwise, the tile this Unit is on.
    pub fn movement_target(&self) -> Option<Tile> {
        self.inner.lock().unwrap()
            .as_unit()
            .movement_target.clone()
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

    /// Moves this Unit from its current Tile to an adjacent Tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile this Unit should move to.
    ///
    /// # Returns
    ///
    /// True if it moved, false otherwise.
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

    /// Harvests the food on an adjacent Tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile you want to harvest.
    ///
    /// # Returns
    ///
    /// True if successfully harvested, false otherwise.
    pub fn harvest(
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
        self.with_context(|cx| cx.run(&self.id(), "harvest", args))
    }

    /// Attacks an adjacent Tile. Costs an action for each Unit in this Unit's squad. Units in the
    /// squad without an action don't participate in combat. Units in combat cannot move
    /// afterwards. Attacking structures will not give materials.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile to attack.
    ///
    /// # Returns
    ///
    /// True if successfully attacked, false otherwise.
    pub fn attack(
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
        self.with_context(|cx| cx.run(&self.id(), "attack", args))
    }

    /// Converts an adjacent Unit to your side.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile with the Unit to convert.
    ///
    /// # Returns
    ///
    /// True if successfully converted, false otherwise.
    pub fn convert(
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
        self.with_context(|cx| cx.run(&self.id(), "convert", args))
    }

    /// Constructs a Structure on an adjacent Tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile to construct the Structure on. It must have enough materials on it for
    /// a Structure to be constructed.
    ///
    /// - _type__ - The type of Structure to construct on that Tile.
    ///
    /// # Returns
    ///
    /// True if successfully constructed a structure, false otherwise.
    pub fn construct(
        &self,
        tile: &Tile,
        type_: &str,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            tile: &'a Tile,
            type_: &'a str,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            tile,
            type_,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "construct", args))
    }

    /// Removes materials from an adjacent Tile's Structure. You cannot deconstruct friendly
    /// structures (see Unit.attack).
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile to deconstruct. It must have a Structure on it.
    ///
    /// # Returns
    ///
    /// True if successfully deconstructed, false otherwise.
    pub fn deconstruct(
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
        self.with_context(|cx| cx.run(&self.id(), "deconstruct", args))
    }

    /// Drops some of the given resource on or adjacent to the Unit's Tile. Does not count as an
    /// action.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile to drop materials/food on.
    ///
    /// - _resource_ - The type of resource to drop ('materials' or 'food').
    ///
    /// - _amount_ - The amount of the resource to drop. Amounts <= 0 will drop as much as
    /// possible.
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

    /// Picks up some materials or food on or adjacent to the Unit's Tile. Does not count as an
    /// action.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile to pickup materials/food from.
    ///
    /// - _resource_ - The type of resource to pickup ('materials' or 'food').
    ///
    /// - _amount_ - The amount of the resource to pickup. Amounts <= 0 will pickup as much as
    /// possible.
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

    /// Changes this Unit's Job. Must be at max energy (100.0) to change Jobs.
    ///
    /// # Arguments
    ///
    /// - _job_ - The name of the Job to change to.
    ///
    /// # Returns
    ///
    /// True if successfully changed Jobs, false otherwise.
    pub fn change_job(
        &self,
        job: &str,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            job: &'a str,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            job,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "changeJob", args))
    }

    /// Regenerates energy. Must be in range of a friendly shelter to rest. Costs an action. Units
    /// cannot move after resting.
    ///
    /// # Returns
    ///
    /// True if successfully rested, false otherwise.
    pub fn rest(
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
        self.with_context(|cx| cx.run(&self.id(), "rest", args))
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

impl inner::ObjectInner for Unit {
    fn from_game_object(game_obj: &Arc<Mutex<inner::AnyGameObject>>, context: &Weak<Mutex<inner::Context>>) -> Option<Self> {
        let handle = game_obj.lock().unwrap();
        if handle.try_as_unit().is_some() {
            Some(Unit {
                inner: Arc::clone(&game_obj),
                context: context.clone(),
            })
        } else {
            None
        }
    }
}
impl Object for Unit {}
