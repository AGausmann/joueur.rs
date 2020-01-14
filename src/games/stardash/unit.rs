#![allow(dead_code, unused_imports)]

use std::any::TypeId;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A unit in the game. May be a corvette, missleboat, martyr, transport, miner.
#[derive(Debug, Clone)]
pub struct Unit {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<UnitInner>>,
}

#[derive(Debug, Clone)]
struct UnitInner {
    unit: Arc<Mutex<UnitBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct UnitBase {
    pub(crate) owner: Option<Player>,
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) dash_x: f64,
    pub(crate) dash_y: f64,
    pub(crate) job: Job,
    pub(crate) energy: i64,
    pub(crate) shield: i64,
    pub(crate) acted: bool,
    pub(crate) moves: f64,
    pub(crate) genarium: i64,
    pub(crate) rarium: i64,
    pub(crate) legendarium: i64,
    pub(crate) mythicite: i64,
    pub(crate) is_busy: bool,
    pub(crate) protector: Option<Unit>,
}

impl Unit {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<UnitInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Unit = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The Player that owns and can control this Unit.
    pub fn owner(&self) -> Option<Player> {
        self.inner().unit.lock().unwrap().owner.clone()
    }

    /// The x value this unit is on.
    pub fn x(&self) -> f64 {
        self.inner().unit.lock().unwrap().x.clone()
    }

    /// The y value this unit is on.
    pub fn y(&self) -> f64 {
        self.inner().unit.lock().unwrap().y.clone()
    }

    /// The x value this unit is dashing to.
    pub fn dash_x(&self) -> f64 {
        self.inner().unit.lock().unwrap().dash_x.clone()
    }

    /// The y value this unit is dashing to.
    pub fn dash_y(&self) -> f64 {
        self.inner().unit.lock().unwrap().dash_y.clone()
    }

    /// The Job this Unit has.
    pub fn job(&self) -> Job {
        self.inner().unit.lock().unwrap().job.clone()
    }

    /// The remaining health of the unit.
    pub fn energy(&self) -> i64 {
        self.inner().unit.lock().unwrap().energy.clone()
    }

    /// The sheild that a martyr ship has.
    pub fn shield(&self) -> i64 {
        self.inner().unit.lock().unwrap().shield.clone()
    }

    /// Whether or not this Unit has performed its action this turn.
    pub fn acted(&self) -> bool {
        self.inner().unit.lock().unwrap().acted.clone()
    }

    /// The distance this unit can still move.
    pub fn moves(&self) -> f64 {
        self.inner().unit.lock().unwrap().moves.clone()
    }

    /// The amount of Genarium ore carried by this unit. (0 to job carry capacity - other carried
    /// items).
    pub fn genarium(&self) -> i64 {
        self.inner().unit.lock().unwrap().genarium.clone()
    }

    /// The amount of Rarium carried by this unit. (0 to job carry capacity - other carried items).
    pub fn rarium(&self) -> i64 {
        self.inner().unit.lock().unwrap().rarium.clone()
    }

    /// The amount of Legendarium ore carried by this unit. (0 to job carry capacity - other
    /// carried items).
    pub fn legendarium(&self) -> i64 {
        self.inner().unit.lock().unwrap().legendarium.clone()
    }

    /// The amount of Mythicite carried by this unit. (0 to job carry capacity - other carried
    /// items).
    pub fn mythicite(&self) -> i64 {
        self.inner().unit.lock().unwrap().mythicite.clone()
    }

    /// Tracks wheither or not the ship is dashing or Mining. If true, it cannot do anything else.
    pub fn is_busy(&self) -> bool {
        self.inner().unit.lock().unwrap().is_busy.clone()
    }

    /// The martyr ship that is currently shielding this ship if any.
    pub fn protector(&self) -> Option<Unit> {
        self.inner().unit.lock().unwrap().protector.clone()
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
        self.context().run(&self.id, "transfer", args)
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
        self.context().run(&self.id, "move", args)
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
        self.context().run(&self.id, "attack", args)
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
        self.context().run(&self.id, "shootdown", args)
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
        self.context().run(&self.id, "mine", args)
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
        self.context().run(&self.id, "safe", args)
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
        self.context().run(&self.id, "dash", args)
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
            x if x == TypeId::of::<Unit>() => Some(T::shallow(self.context.clone(), self.id.clone())),
            x if x == TypeId::of::<GameObject>() => Some(T::shallow(self.context.clone(), self.id.clone())),
            _ => None,
        }
    }
}

impl ObjectInner for Unit {
    fn shallow(context: Weak<Context>, id: Str) -> Unit {
        Unit {
            context,
            id,
            inner: RefCell::new(None),
        }
    }
}

impl Object for Unit {}
