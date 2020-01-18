#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A unit in the game. May be a worker, zombie, ghoul, hound, abomination, wraith or horseman.
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

    /// The Player that owns and can control this Unit.
    pub fn owner(&self) -> Option<Player> {
        self.inner.lock().unwrap().as_unit()
            .owner.clone()
    }

    /// The Tile this Unit is on.
    pub fn tile(&self) -> Option<Tile> {
        self.inner.lock().unwrap().as_unit()
            .tile.clone()
    }

    /// The type of unit this is.
    pub fn job(&self) -> UnitJob {
        self.inner.lock().unwrap().as_unit()
            .job.clone()
    }

    /// The remaining health of a unit.
    pub fn health(&self) -> i64 {
        self.inner.lock().unwrap().as_unit()
            .health.clone()
    }

    /// Whether or not this Unit has performed its action this turn (attack or build).
    pub fn acted(&self) -> bool {
        self.inner.lock().unwrap().as_unit()
            .acted.clone()
    }

    /// The number of moves this unit has left this turn.
    pub fn moves(&self) -> i64 {
        self.inner.lock().unwrap().as_unit()
            .moves.clone()
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

    /// Enters a mine and is put to work gathering resources.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The tile the mine is located on.
    ///
    /// # Returns
    ///
    /// True if successfully entered mine and began mining, false otherwise.
    pub fn mine(
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
        self.with_context(|cx| cx.run(&self.id(), "mine", args))
    }

    /// Stops adjacent to a river tile and begins fishing for mana.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The tile the unit will stand on as it fishes.
    ///
    /// # Returns
    ///
    /// True if successfully began fishing for mana, false otherwise.
    pub fn fish(
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
        self.with_context(|cx| cx.run(&self.id(), "fish", args))
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

    /// Attacks an enemy tower on an adjacent tile.
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

    /// Unit, if it is a worker, builds a tower on the tile it is on, only workers can do this.
    ///
    /// # Arguments
    ///
    /// - _title_ - The tower type to build, as a string.
    ///
    /// # Returns
    ///
    /// True if successfully built, false otherwise.
    pub fn build(
        &self,
        title: &str,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            title: &'a str,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            title,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "build", args))
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
}
