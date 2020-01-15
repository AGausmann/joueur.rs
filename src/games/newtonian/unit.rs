#![allow(dead_code, unused_imports)]

use std::any::TypeId;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A unit in the game. May be a manager, intern, or physicist.
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
    pub(crate) tile: Option<Tile>,
    pub(crate) job: Job,
    pub(crate) health: i64,
    pub(crate) acted: bool,
    pub(crate) moves: i64,
    pub(crate) redium_ore: i64,
    pub(crate) redium: i64,
    pub(crate) blueium_ore: i64,
    pub(crate) blueium: i64,
    pub(crate) stun_time: i64,
    pub(crate) stun_immune: i64,
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

    /// The Tile this Unit is on.
    pub fn tile(&self) -> Option<Tile> {
        self.inner().unit.lock().unwrap().tile.clone()
    }

    /// The Job this Unit has.
    pub fn job(&self) -> Job {
        self.inner().unit.lock().unwrap().job.clone()
    }

    /// The remaining health of a unit.
    pub fn health(&self) -> i64 {
        self.inner().unit.lock().unwrap().health.clone()
    }

    /// Whether or not this Unit has performed its action this turn.
    pub fn acted(&self) -> bool {
        self.inner().unit.lock().unwrap().acted.clone()
    }

    /// The number of moves this unit has left this turn.
    pub fn moves(&self) -> i64 {
        self.inner().unit.lock().unwrap().moves.clone()
    }

    /// The amount of redium ore carried by this unit. (0 to job carry capacity - other carried
    /// items).
    pub fn redium_ore(&self) -> i64 {
        self.inner().unit.lock().unwrap().redium_ore.clone()
    }

    /// The amount of redium carried by this unit. (0 to job carry capacity - other carried items).
    pub fn redium(&self) -> i64 {
        self.inner().unit.lock().unwrap().redium.clone()
    }

    /// The amount of blueium ore carried by this unit. (0 to job carry capacity - other carried
    /// items).
    pub fn blueium_ore(&self) -> i64 {
        self.inner().unit.lock().unwrap().blueium_ore.clone()
    }

    /// The amount of blueium carried by this unit. (0 to job carry capacity - other carried
    /// items).
    pub fn blueium(&self) -> i64 {
        self.inner().unit.lock().unwrap().blueium.clone()
    }

    /// Duration the unit is stunned. (0 to the game constant stunTime).
    pub fn stun_time(&self) -> i64 {
        self.inner().unit.lock().unwrap().stun_time.clone()
    }

    /// Duration of stun immunity. (0 to timeImmune).
    pub fn stun_immune(&self) -> i64 {
        self.inner().unit.lock().unwrap().stun_immune.clone()
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

    /// Drops materials at the units feet or adjacent tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The tile the materials will be dropped on.
    ///
    /// - _amount_ - The number of materials to dropped. Amounts <= 0 will drop all the materials.
    ///
    /// - _material_ - The material the unit will drop. 'redium', 'blueium', 'redium ore', or
    /// 'blueium ore'.
    ///
    /// # Returns
    ///
    /// True if successfully deposited, false otherwise.
    pub fn drop(
        &self,
        tile: &Tile,
        amount: i64,
        material: &str,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            tile: &'a Tile,
            amount: i64,
            material: &'a str,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            tile,
            amount,
            material,
            _a: PhantomData,
        };
        self.context().run(&self.id, "drop", args)
    }

    /// Picks up material at the units feet or adjacent tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The tile the materials will be picked up from.
    ///
    /// - _amount_ - The amount of materials to pick up. Amounts <= 0 will pick up all the
    /// materials that the unit can.
    ///
    /// - _material_ - The material the unit will pick up. 'redium', 'blueium', 'redium ore', or
    /// 'blueium ore'.
    ///
    /// # Returns
    ///
    /// True if successfully deposited, false otherwise.
    pub fn pickup(
        &self,
        tile: &Tile,
        amount: i64,
        material: &str,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            tile: &'a Tile,
            amount: i64,
            material: &'a str,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            tile,
            amount,
            material,
            _a: PhantomData,
        };
        self.context().run(&self.id, "pickup", args)
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
        self.context().run(&self.id, "move", args)
    }

    /// Attacks a unit on an adjacent tile.
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
        self.context().run(&self.id, "attack", args)
    }

    /// Makes the unit do something to a machine or unit adjacent to its tile. Interns sabotage,
    /// physicists work. Interns stun physicist, physicist stuns manager, manager stuns intern.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The tile the unit acts on.
    ///
    /// # Returns
    ///
    /// True if successfully acted, false otherwise.
    pub fn act(
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
}

impl ObjectInner for Unit {
    fn to_bases(&self) -> Bases {
        let inner = self.inner();
        Bases {
            context: Some(self.context.clone()),
            id: Some(self.id.clone()),
            unit: Some(Arc::clone(&inner.unit)),
            game_object: Some(Arc::clone(&inner.game_object)),
            ..Default::default()
        }
    }

    fn from_bases(bases: Bases) -> Option<Self> {
        let inner = UnitInner {
            unit: bases.unit?,
            game_object: bases.game_object?,
        };

        Some(Unit {
            context: bases.context?,
            id: bases.id?,
            inner: RefCell::new(Some(inner)),
        })
    }
}

impl Object for Unit {}
