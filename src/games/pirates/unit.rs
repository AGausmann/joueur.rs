#![allow(dead_code, unused_imports)]

use std::any::TypeId;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A unit group in the game. This may consist of a ship and any number of crew.
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
    pub(crate) ship_health: i64,
    pub(crate) crew: i64,
    pub(crate) crew_health: i64,
    pub(crate) gold: i64,
    pub(crate) acted: bool,
    pub(crate) moves: i64,
    pub(crate) path: List<Tile>,
    pub(crate) target_port: Option<Port>,
    pub(crate) stun_turns: i64,
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


    /// The Player that owns and can control this Unit, or None if the Unit is neutral.
    pub fn owner(&self) -> Option<Player> {
        self.inner().unit.lock().unwrap().owner.clone()
    }

    /// The Tile this Unit is on.
    pub fn tile(&self) -> Option<Tile> {
        self.inner().unit.lock().unwrap().tile.clone()
    }

    /// If a ship is on this Tile, how much health it has remaining. 0 for no ship.
    pub fn ship_health(&self) -> i64 {
        self.inner().unit.lock().unwrap().ship_health.clone()
    }

    /// How many crew are on this Tile. This number will always be <= crewHealth.
    pub fn crew(&self) -> i64 {
        self.inner().unit.lock().unwrap().crew.clone()
    }

    /// How much total health the crew on this Tile have.
    pub fn crew_health(&self) -> i64 {
        self.inner().unit.lock().unwrap().crew_health.clone()
    }

    /// How much gold this Unit is carrying.
    pub fn gold(&self) -> i64 {
        self.inner().unit.lock().unwrap().gold.clone()
    }

    /// Whether this Unit has performed its action this turn.
    pub fn acted(&self) -> bool {
        self.inner().unit.lock().unwrap().acted.clone()
    }

    /// How many more times this Unit may move this turn.
    pub fn moves(&self) -> i64 {
        self.inner().unit.lock().unwrap().moves.clone()
    }

    /// (Merchants only) The path this Unit will follow. The first element is the Tile this Unit
    /// will move to next.
    pub fn path(&self) -> List<Tile> {
        self.inner().unit.lock().unwrap().path.clone()
    }

    /// (Merchants only) The Port this Unit is moving to.
    pub fn target_port(&self) -> Option<Port> {
        self.inner().unit.lock().unwrap().target_port.clone()
    }

    /// (Merchants only) The number of turns this merchant ship won't be able to move. They will
    /// still attack. Merchant ships are stunned when they're attacked.
    pub fn stun_turns(&self) -> i64 {
        self.inner().unit.lock().unwrap().stun_turns.clone()
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

    /// Moves this Unit from its current Tile to an adjacent Tile. If this Unit merges with another
    /// one, the other Unit will be destroyed and its tile will be set to None. Make sure to check
    /// that your Unit's tile is not None before doing things with it.
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

    /// Attacks either the 'crew' or 'ship' on a Tile in range.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile to attack.
    ///
    /// - _target_ - Whether to attack 'crew' or 'ship'. Crew deal damage to crew and ships deal
    /// damage to ships. Consumes any remaining moves.
    ///
    /// # Returns
    ///
    /// True if successfully attacked, false otherwise.
    pub fn attack(
        &self,
        tile: &Tile,
        target: &str,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            tile: &'a Tile,
            target: &'a str,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            tile,
            target,
            _a: PhantomData,
        };
        self.context().run(&self.id, "attack", args)
    }

    /// Buries gold on this Unit's Tile. Gold must be a certain distance away for it to get
    /// interest (Game.minInterestDistance).
    ///
    /// # Arguments
    ///
    /// - _amount_ - How much gold this Unit should bury. Amounts <= 0 will bury as much as
    /// possible.
    ///
    /// # Returns
    ///
    /// True if successfully buried, false otherwise.
    pub fn bury(
        &self,
        amount: i64,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            amount: i64,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            amount,
            _a: PhantomData,
        };
        self.context().run(&self.id, "bury", args)
    }

    /// Digs up gold on this Unit's Tile.
    ///
    /// # Arguments
    ///
    /// - _amount_ - How much gold this Unit should take. Amounts <= 0 will dig up as much as
    /// possible.
    ///
    /// # Returns
    ///
    /// True if successfully dug up, false otherwise.
    pub fn dig(
        &self,
        amount: i64,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            amount: i64,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            amount,
            _a: PhantomData,
        };
        self.context().run(&self.id, "dig", args)
    }

    /// Puts gold into an adjacent Port. If that Port is the Player's port, the gold is added to
    /// that Player. If that Port is owned by merchants, it adds to that Port's investment.
    ///
    /// # Arguments
    ///
    /// - _amount_ - The amount of gold to deposit. Amounts <= 0 will deposit all the gold on this
    /// Unit.
    ///
    /// # Returns
    ///
    /// True if successfully deposited, false otherwise.
    pub fn deposit(
        &self,
        amount: i64,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            amount: i64,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            amount,
            _a: PhantomData,
        };
        self.context().run(&self.id, "deposit", args)
    }

    /// Takes gold from the Player. You can only withdraw from your own Port.
    ///
    /// # Arguments
    ///
    /// - _amount_ - The amount of gold to withdraw. Amounts <= 0 will withdraw everything.
    ///
    /// # Returns
    ///
    /// True if successfully withdrawn, false otherwise.
    pub fn withdraw(
        &self,
        amount: i64,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            amount: i64,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            amount,
            _a: PhantomData,
        };
        self.context().run(&self.id, "withdraw", args)
    }

    /// Moves a number of crew from this Unit to the given Tile. This will consume a move from
    /// those crew.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile to move the crew to.
    ///
    /// - _amount_ - The number of crew to move onto that Tile. Amount <= 0 will move all the crew
    /// to that Tile.
    ///
    /// - _gold_ - The amount of gold the crew should take with them. Gold < 0 will move all the
    /// gold to that Tile.
    ///
    /// # Returns
    ///
    /// True if successfully split, false otherwise.
    pub fn split(
        &self,
        tile: &Tile,
        amount: i64,
        gold: i64,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            tile: &'a Tile,
            amount: i64,
            gold: i64,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            tile,
            amount,
            gold,
            _a: PhantomData,
        };
        self.context().run(&self.id, "split", args)
    }

    /// Regenerates this Unit's health. Must be used in range of a port.
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
        self.context().run(&self.id, "rest", args)
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
