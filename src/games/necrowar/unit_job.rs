#![allow(dead_code, unused_imports)]

use std::any::TypeId;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// Information about a unit's job/type.
#[derive(Debug, Clone)]
pub struct UnitJob {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<UnitJobInner>>,
}

#[derive(Debug, Clone)]
struct UnitJobInner {
    unit_job: Arc<Mutex<UnitJobBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct UnitJobBase {
    pub(crate) title: Str,
    pub(crate) per_tile: i64,
    pub(crate) health: i64,
    pub(crate) moves: i64,
    pub(crate) damage: i64,
    pub(crate) gold_cost: i64,
    pub(crate) mana_cost: i64,
    pub(crate) range: i64,
}

impl UnitJob {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<UnitJobInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: UnitJob = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The type title. 'worker', 'zombie', 'ghoul', 'hound', 'abomination', 'wraith' or
    /// 'horseman'.
    pub fn title(&self) -> Str {
        self.inner().unit_job.lock().unwrap().title.clone()
    }

    /// How many of this type of unit can take up one tile.
    pub fn per_tile(&self) -> i64 {
        self.inner().unit_job.lock().unwrap().per_tile.clone()
    }

    /// The amount of starting health this type has.
    pub fn health(&self) -> i64 {
        self.inner().unit_job.lock().unwrap().health.clone()
    }

    /// The number of moves this type can make per turn.
    pub fn moves(&self) -> i64 {
        self.inner().unit_job.lock().unwrap().moves.clone()
    }

    /// The amount of damage this type does per attack.
    pub fn damage(&self) -> i64 {
        self.inner().unit_job.lock().unwrap().damage.clone()
    }

    /// How much does this type cost in gold.
    pub fn gold_cost(&self) -> i64 {
        self.inner().unit_job.lock().unwrap().gold_cost.clone()
    }

    /// How much does this type cost in mana.
    pub fn mana_cost(&self) -> i64 {
        self.inner().unit_job.lock().unwrap().mana_cost.clone()
    }

    /// Amount of tiles away this type has to be in order to be effective.
    pub fn range(&self) -> i64 {
        self.inner().unit_job.lock().unwrap().range.clone()
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
            x if x == TypeId::of::<UnitJob>() => Some(T::shallow(self.context.clone(), self.id.clone())),
            x if x == TypeId::of::<GameObject>() => Some(T::shallow(self.context.clone(), self.id.clone())),
            _ => None,
        }
    }
}

impl ObjectInner for UnitJob {
    fn shallow(context: Weak<Context>, id: Str) -> UnitJob {
        UnitJob {
            context,
            id,
            inner: RefCell::new(None),
        }
    }
}

impl Object for UnitJob {}
