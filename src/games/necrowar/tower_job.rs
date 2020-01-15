#![allow(dead_code, unused_imports)]

use std::any::TypeId;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// Information about a tower's job/type.
#[derive(Debug, Clone)]
pub struct TowerJob {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<TowerJobInner>>,
}

#[derive(Debug, Clone)]
struct TowerJobInner {
    tower_job: Arc<Mutex<TowerJobBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct TowerJobBase {
    pub(crate) title: Str,
    pub(crate) health: i64,
    pub(crate) range: i64,
    pub(crate) all_units: bool,
    pub(crate) damage: i64,
    pub(crate) gold_cost: i64,
    pub(crate) turns_between_attacks: i64,
    pub(crate) mana_cost: i64,
}

impl TowerJob {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<TowerJobInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: TowerJob = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The type title. 'arrow', 'aoe', 'ballista', 'cleansing', or 'castle'.
    pub fn title(&self) -> Str {
        self.inner().tower_job.lock().unwrap().title.clone()
    }

    /// The amount of starting health this type has.
    pub fn health(&self) -> i64 {
        self.inner().tower_job.lock().unwrap().health.clone()
    }

    /// The number of tiles this type can attack from.
    pub fn range(&self) -> i64 {
        self.inner().tower_job.lock().unwrap().range.clone()
    }

    /// Whether this tower type hits all of the units on a tile (true) or one at a time (false).
    pub fn all_units(&self) -> bool {
        self.inner().tower_job.lock().unwrap().all_units.clone()
    }

    /// The amount of damage this type does per attack.
    pub fn damage(&self) -> i64 {
        self.inner().tower_job.lock().unwrap().damage.clone()
    }

    /// How much does this type cost in gold.
    pub fn gold_cost(&self) -> i64 {
        self.inner().tower_job.lock().unwrap().gold_cost.clone()
    }

    /// How many turns have to take place between this type's attacks.
    pub fn turns_between_attacks(&self) -> i64 {
        self.inner().tower_job.lock().unwrap().turns_between_attacks.clone()
    }

    /// How much does this type cost in mana.
    pub fn mana_cost(&self) -> i64 {
        self.inner().tower_job.lock().unwrap().mana_cost.clone()
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

impl ObjectInner for TowerJob {
    fn to_bases(&self) -> Bases {
        let inner = self.inner();
        Bases {
            context: Some(self.context.clone()),
            id: Some(self.id.clone()),
            tower_job: Some(Arc::clone(&inner.tower_job)),
            game_object: Some(Arc::clone(&inner.game_object)),
            ..Default::default()
        }
    }

    fn from_bases(bases: Bases) -> Option<Self> {
        let inner = TowerJobInner {
            tower_job: bases.tower_job?,
            game_object: bases.game_object?,
        };

        Some(TowerJob {
            context: bases.context?,
            id: bases.id?,
            inner: RefCell::new(Some(inner)),
        })
    }
}

impl Object for TowerJob {}
