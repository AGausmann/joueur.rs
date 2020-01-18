#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A unit group in the game. This may consist of a ship and any number of crew.
#[derive(Debug, Clone)]
pub struct Unit {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::GameObject>>,
}

impl Unit {
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
        self.inner.lock().unwrap().as_unit()
            .owner.clone()
    }

    /// The Tile this Unit is on.
    pub fn tile(&self) -> Option<Tile> {
        self.inner.lock().unwrap().as_unit()
            .tile.clone()
    }

    /// If a ship is on this Tile, how much health it has remaining. 0 for no ship.
    pub fn ship_health(&self) -> i64 {
        self.inner.lock().unwrap().as_unit()
            .ship_health.clone()
    }

    /// How many crew are on this Tile. This number will always be <= crewHealth.
    pub fn crew(&self) -> i64 {
        self.inner.lock().unwrap().as_unit()
            .crew.clone()
    }

    /// How much total health the crew on this Tile have.
    pub fn crew_health(&self) -> i64 {
        self.inner.lock().unwrap().as_unit()
            .crew_health.clone()
    }

    /// How much gold this Unit is carrying.
    pub fn gold(&self) -> i64 {
        self.inner.lock().unwrap().as_unit()
            .gold.clone()
    }

    /// Whether this Unit has performed its action this turn.
    pub fn acted(&self) -> bool {
        self.inner.lock().unwrap().as_unit()
            .acted.clone()
    }

    /// How many more times this Unit may move this turn.
    pub fn moves(&self) -> i64 {
        self.inner.lock().unwrap().as_unit()
            .moves.clone()
    }

    /// (Merchants only) The path this Unit will follow. The first element is the Tile this Unit
    /// will move to next.
    pub fn path(&self) -> List<Tile> {
        self.inner.lock().unwrap().as_unit()
            .path.clone()
    }

    /// (Merchants only) The Port this Unit is moving to.
    pub fn target_port(&self) -> Option<Port> {
        self.inner.lock().unwrap().as_unit()
            .target_port.clone()
    }

    /// (Merchants only) The number of turns this merchant ship won't be able to move. They will
    /// still attack. Merchant ships are stunned when they're attacked.
    pub fn stun_turns(&self) -> i64 {
        self.inner.lock().unwrap().as_unit()
            .stun_turns.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// A unique id for each instance of a GameObject or a sub class. Used for client and server
    /// communication. Should never change value after being set.
    pub fn id(&self) -> Str {
        self.inner.lock().unwrap().as_game_object()
            .id.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// String representing the top level Class that this game object is an instance of. Used for
    /// reflection to create new instances on clients, but exposed for convenience should AIs want
    /// this data.
    pub fn game_object_name(&self) -> Str {
        self.inner.lock().unwrap().as_game_object()
            .game_object_name.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// Any strings logged will be stored here. Intended for debugging.
    pub fn logs(&self) -> List<Str> {
        self.inner.lock().unwrap().as_game_object()
            .logs.clone()
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
        self.with_context(|cx| cx.run(&self.id(), "move", args))
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
        self.with_context(|cx| cx.run(&self.id(), "attack", args))
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
        self.with_context(|cx| cx.run(&self.id(), "bury", args))
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
        self.with_context(|cx| cx.run(&self.id(), "dig", args))
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
        self.with_context(|cx| cx.run(&self.id(), "deposit", args))
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
        self.with_context(|cx| cx.run(&self.id(), "withdraw", args))
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
        self.with_context(|cx| cx.run(&self.id(), "split", args))
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
    fn from_game_object(game_obj: &Arc<Mutex<inner::GameObject>>, context: &Weak<Mutex<inner::Context>>) -> Option<Self> {
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
