#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// Information about a unit's job/type.
#[derive(Debug, Clone)]
pub struct UnitJob {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::AnyGameObject>>,
}

impl UnitJob {
    pub(crate) fn new(inner: Arc<Mutex<inner::AnyGameObject>>, context: Weak<Mutex<inner::Context>>) -> UnitJob {
        UnitJob { inner, context }
    }

    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }

    /// The type title. 'worker', 'zombie', 'ghoul', 'hound', 'abomination', 'wraith' or
    /// 'horseman'.
    pub fn title(&self) -> Str {
        self.inner.lock().unwrap()
            .as_unit_job()
            .title.clone()
    }

    /// How many of this type of unit can take up one tile.
    pub fn per_tile(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_unit_job()
            .per_tile.clone()
    }

    /// The amount of starting health this type has.
    pub fn health(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_unit_job()
            .health.clone()
    }

    /// The number of moves this type can make per turn.
    pub fn moves(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_unit_job()
            .moves.clone()
    }

    /// The amount of damage this type does per attack.
    pub fn damage(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_unit_job()
            .damage.clone()
    }

    /// How much does this type cost in gold.
    pub fn gold_cost(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_unit_job()
            .gold_cost.clone()
    }

    /// How much does this type cost in mana.
    pub fn mana_cost(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_unit_job()
            .mana_cost.clone()
    }

    /// Amount of tiles away this type has to be in order to be effective.
    pub fn range(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_unit_job()
            .range.clone()
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

impl inner::ObjectInner for UnitJob {
    fn from_game_object(game_obj: &Arc<Mutex<inner::AnyGameObject>>, context: &Weak<Mutex<inner::Context>>) -> Option<Self> {
        let handle = game_obj.lock().unwrap();
        if handle.try_as_unit_job().is_some() {
            Some(UnitJob {
                inner: Arc::clone(&game_obj),
                context: context.clone(),
            })
        } else {
            None
        }
    }
}
impl Object for UnitJob {}
