#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A unit in the game. May be a corvette, missleboat, martyr, transport, miner.
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

    /// The Player that owns and can control this Unit.
    pub fn owner(&self) -> Option<Player> {
        self.inner.lock().unwrap()
            .as_unit()
            .owner.clone()
    }

    /// The x value this unit is on.
    pub fn x(&self) -> f64 {
        self.inner.lock().unwrap()
            .as_unit()
            .x.clone()
    }

    /// The y value this unit is on.
    pub fn y(&self) -> f64 {
        self.inner.lock().unwrap()
            .as_unit()
            .y.clone()
    }

    /// The x value this unit is dashing to.
    pub fn dash_x(&self) -> f64 {
        self.inner.lock().unwrap()
            .as_unit()
            .dash_x.clone()
    }

    /// The y value this unit is dashing to.
    pub fn dash_y(&self) -> f64 {
        self.inner.lock().unwrap()
            .as_unit()
            .dash_y.clone()
    }

    /// The Job this Unit has.
    pub fn job(&self) -> Job {
        self.inner.lock().unwrap()
            .as_unit()
            .job.clone()
    }

    /// The remaining health of the unit.
    pub fn energy(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_unit()
            .energy.clone()
    }

    /// The sheild that a martyr ship has.
    pub fn shield(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_unit()
            .shield.clone()
    }

    /// Whether or not this Unit has performed its action this turn.
    pub fn acted(&self) -> bool {
        self.inner.lock().unwrap()
            .as_unit()
            .acted.clone()
    }

    /// The distance this unit can still move.
    pub fn moves(&self) -> f64 {
        self.inner.lock().unwrap()
            .as_unit()
            .moves.clone()
    }

    /// The amount of Genarium ore carried by this unit. (0 to job carry capacity - other carried
    /// items).
    pub fn genarium(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_unit()
            .genarium.clone()
    }

    /// The amount of Rarium carried by this unit. (0 to job carry capacity - other carried items).
    pub fn rarium(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_unit()
            .rarium.clone()
    }

    /// The amount of Legendarium ore carried by this unit. (0 to job carry capacity - other
    /// carried items).
    pub fn legendarium(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_unit()
            .legendarium.clone()
    }

    /// The amount of Mythicite carried by this unit. (0 to job carry capacity - other carried
    /// items).
    pub fn mythicite(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_unit()
            .mythicite.clone()
    }

    /// Tracks wheither or not the ship is dashing or Mining. If true, it cannot do anything else.
    pub fn is_busy(&self) -> bool {
        self.inner.lock().unwrap()
            .as_unit()
            .is_busy.clone()
    }

    /// The martyr ship that is currently shielding this ship if any.
    pub fn protector(&self) -> Option<Unit> {
        self.inner.lock().unwrap()
            .as_unit()
            .protector.clone()
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

    /// Grab materials from a friendly unit. Doesn't use a action.
    ///
    /// # Arguments
    ///
    /// - _unit_ - The unit you are grabbing the resources from.
    ///
    /// - _amount_ - The amount of materials to you with to grab. Amounts <= 0 will pick up all the
    /// materials that the unit can.
    ///
    /// - _material_ - The material the unit will pick up. 'genarium', 'rarium', 'legendarium', or
    /// 'mythicite'.
    ///
    /// # Returns
    ///
    /// True if successfully taken, false otherwise.
    pub fn transfer(
        &self,
        unit: &Unit,
        amount: i64,
        material: &str,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            unit: &'a Unit,
            amount: i64,
            material: &'a str,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            unit,
            amount,
            material,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "transfer", args))
    }

    /// Moves this Unit from its current location to the new location specified.
    ///
    /// # Arguments
    ///
    /// - _x_ - The x value of the destination's coordinates.
    ///
    /// - _y_ - The y value of the destination's coordinates.
    ///
    /// # Returns
    ///
    /// True if it moved, false otherwise.
    pub fn move_(
        &self,
        x: f64,
        y: f64,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            x: f64,
            y: f64,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            x,
            y,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "move", args))
    }

    /// Attacks the specified unit.
    ///
    /// # Arguments
    ///
    /// - _enemy_ - The Unit being attacked.
    ///
    /// # Returns
    ///
    /// True if successfully attacked, false otherwise.
    pub fn attack(
        &self,
        enemy: &Unit,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            enemy: &'a Unit,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            enemy,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "attack", args))
    }

    /// Attacks the specified projectile.
    ///
    /// # Arguments
    ///
    /// - _missile_ - The projectile being shot down.
    ///
    /// # Returns
    ///
    /// True if successfully attacked, false otherwise.
    pub fn shootdown(
        &self,
        missile: &Projectile,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            missile: &'a Projectile,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            missile,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "shootdown", args))
    }

    /// allows a miner to mine a asteroid
    ///
    /// # Arguments
    ///
    /// - _body_ - The object to be mined.
    ///
    /// # Returns
    ///
    /// True if successfully acted, false otherwise.
    pub fn mine(
        &self,
        body: &Body,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            body: &'a Body,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            body,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "mine", args))
    }

    /// tells you if your ship can move to that location from were it is without clipping the sun.
    ///
    /// # Arguments
    ///
    /// - _x_ - The x position of the location you wish to arrive.
    ///
    /// - _y_ - The y position of the location you wish to arrive.
    ///
    /// # Returns
    ///
    /// True if pathable by this unit, false otherwise.
    pub fn safe(
        &self,
        x: f64,
        y: f64,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            x: f64,
            y: f64,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            x,
            y,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "safe", args))
    }

    /// Causes the unit to dash towards the designated destination.
    ///
    /// # Arguments
    ///
    /// - _x_ - The x value of the destination's coordinates.
    ///
    /// - _y_ - The y value of the destination's coordinates.
    ///
    /// # Returns
    ///
    /// True if it moved, false otherwise.
    pub fn dash(
        &self,
        x: f64,
        y: f64,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            x: f64,
            y: f64,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            x,
            y,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "dash", args))
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
