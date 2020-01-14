#![allow(dead_code, unused_imports)]

use std::any::TypeId;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// Information about a Unit's job.
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
    pub(crate) moves: i64,
    pub(crate) action_cost: f64,
    pub(crate) regen_rate: f64,
    pub(crate) carry_limit: i64,
    pub(crate) upkeep: i64,
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

    /// The number of moves this Job can make per turn.
    pub fn moves(&self) -> i64 {
        self.inner().job.lock().unwrap().moves.clone()
    }

    /// The amount of energy this Job normally uses to perform its actions.
    pub fn action_cost(&self) -> f64 {
        self.inner().job.lock().unwrap().action_cost.clone()
    }

    /// The amount of energy normally regenerated when resting at a shelter.
    pub fn regen_rate(&self) -> f64 {
        self.inner().job.lock().unwrap().regen_rate.clone()
    }

    /// How many combined resources a Unit with this Job can hold at once.
    pub fn carry_limit(&self) -> i64 {
        self.inner().job.lock().unwrap().carry_limit.clone()
    }

    /// The amount of food per turn this Unit consumes. If there isn't enough food for every Unit,
    /// all Units become starved and do not consume food.
    pub fn upkeep(&self) -> i64 {
        self.inner().job.lock().unwrap().upkeep.clone()
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

    pub(crate) fn try_upcast<T: Object>(&self) -> Option<T> {
        match TypeId::of::<T>() {
            x if x == TypeId::of::<Job>() => Some(T::shallow(self.context.clone(), self.id.clone())),
            x if x == TypeId::of::<GameObject>() => Some(T::shallow(self.context.clone(), self.id.clone())),
            _ => None,
        }
    }
}

impl ObjectInner for Job {
    fn shallow(context: Weak<Context>, id: Str) -> Job {
        Job {
            context,
            id,
            inner: RefCell::new(None),
        }
    }
}

impl Object for Job {}
