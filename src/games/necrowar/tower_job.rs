#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// Information about a tower's job/type.
#[derive(Debug, Clone)]
pub struct TowerJob {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::GameObject>>,
}

impl TowerJob {
    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }

    /// The type title. 'arrow', 'aoe', 'ballista', 'cleansing', or 'castle'.
    pub fn title(&self) -> Str {
        self.inner.lock().unwrap().as_tower_job()
            .title.clone()
    }

    /// The amount of starting health this type has.
    pub fn health(&self) -> i64 {
        self.inner.lock().unwrap().as_tower_job()
            .health.clone()
    }

    /// The number of tiles this type can attack from.
    pub fn range(&self) -> i64 {
        self.inner.lock().unwrap().as_tower_job()
            .range.clone()
    }

    /// Whether this tower type hits all of the units on a tile (true) or one at a time (false).
    pub fn all_units(&self) -> bool {
        self.inner.lock().unwrap().as_tower_job()
            .all_units.clone()
    }

    /// The amount of damage this type does per attack.
    pub fn damage(&self) -> i64 {
        self.inner.lock().unwrap().as_tower_job()
            .damage.clone()
    }

    /// How much does this type cost in gold.
    pub fn gold_cost(&self) -> i64 {
        self.inner.lock().unwrap().as_tower_job()
            .gold_cost.clone()
    }

    /// How many turns have to take place between this type's attacks.
    pub fn turns_between_attacks(&self) -> i64 {
        self.inner.lock().unwrap().as_tower_job()
            .turns_between_attacks.clone()
    }

    /// How much does this type cost in mana.
    pub fn mana_cost(&self) -> i64 {
        self.inner.lock().unwrap().as_tower_job()
            .mana_cost.clone()
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

impl inner::ObjectInner for TowerJob {
    fn from_game_object(game_obj: &Arc<Mutex<inner::GameObject>>, context: &Weak<Mutex<inner::Context>>) -> Option<Self> {
        let handle = game_obj.lock().unwrap();
        if handle.try_as_tower_job().is_some() {
            Some(TowerJob {
                inner: Arc::clone(&game_obj),
                context: context.clone(),
            })
        } else {
            None
        }
    }
}
impl Object for TowerJob {}
