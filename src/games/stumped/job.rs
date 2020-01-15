#![allow(dead_code, unused_imports)]

use std::any::TypeId;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// Information about a beaver's job.
#[derive(Debug, Clone)]
pub struct Job {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<JobInner>>,
}

#[derive(Debug, Clone)]
struct JobInner {
    job: Arc<Mutex<JobBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct JobBase {
    pub(crate) title: Str,
    pub(crate) health: i64,
    pub(crate) moves: i64,
    pub(crate) actions: i64,
    pub(crate) damage: i64,
    pub(crate) chopping: i64,
    pub(crate) munching: i64,
    pub(crate) distraction_power: i64,
    pub(crate) carry_limit: i64,
    pub(crate) cost: i64,
}

impl Job {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<JobInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Job = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The Job title.
    pub fn title(&self) -> Str {
        self.inner().job.lock().unwrap().title.clone()
    }

    /// The amount of starting health this Job has.
    pub fn health(&self) -> i64 {
        self.inner().job.lock().unwrap().health.clone()
    }

    /// The number of moves this Job can make per turn.
    pub fn moves(&self) -> i64 {
        self.inner().job.lock().unwrap().moves.clone()
    }

    /// The number of actions this Job can make per turn.
    pub fn actions(&self) -> i64 {
        self.inner().job.lock().unwrap().actions.clone()
    }

    /// The amount of damage this Job does per attack.
    pub fn damage(&self) -> i64 {
        self.inner().job.lock().unwrap().damage.clone()
    }

    /// Scalar for how many branches this Job harvests at once.
    pub fn chopping(&self) -> i64 {
        self.inner().job.lock().unwrap().chopping.clone()
    }

    /// Scalar for how much food this Job harvests at once.
    pub fn munching(&self) -> i64 {
        self.inner().job.lock().unwrap().munching.clone()
    }

    /// How many turns a beaver attacked by this Job is distracted by.
    pub fn distraction_power(&self) -> i64 {
        self.inner().job.lock().unwrap().distraction_power.clone()
    }

    /// How many combined resources a beaver with this Job can hold at once.
    pub fn carry_limit(&self) -> i64 {
        self.inner().job.lock().unwrap().carry_limit.clone()
    }

    /// How much food this Job costs to recruit.
    pub fn cost(&self) -> i64 {
        self.inner().job.lock().unwrap().cost.clone()
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

    /// Recruits a Beaver of this Job to a lodge
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile that is a lodge owned by you that you wish to spawn the Beaver of this
    /// Job on.
    ///
    /// # Returns
    ///
    /// The recruited Beaver if successful, None otherwise.
    pub fn recruit(
        &self,
        tile: &Tile,
    )
        -> Result<Option<Beaver>, Error>
    {
        struct Args<'a> {
            tile: &'a Tile,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            tile,
            _a: PhantomData,
        };
        self.context().run(&self.id, "recruit", args)
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

impl ObjectInner for Job {
    fn to_bases(&self) -> Bases {
        let inner = self.inner();
        Bases {
            context: Some(self.context.clone()),
            id: Some(self.id.clone()),
            job: Some(Arc::clone(&inner.job)),
            game_object: Some(Arc::clone(&inner.game_object)),
            ..Default::default()
        }
    }

    fn from_bases(bases: Bases) -> Option<Self> {
        let inner = JobInner {
            job: bases.job?,
            game_object: bases.game_object?,
        };

        Some(Job {
            context: bases.context?,
            id: bases.id?,
            inner: RefCell::new(Some(inner)),
        })
    }
}

impl Object for Job {}
