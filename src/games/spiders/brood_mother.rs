#![allow(dead_code, unused_imports)]

use std::any::TypeId;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// The Spider Queen. She alone can spawn Spiderlings for each Player, and if she dies the owner
/// loses.
#[derive(Debug, Clone)]
pub struct BroodMother {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<BroodMotherInner>>,
}

#[derive(Debug, Clone)]
struct BroodMotherInner {
    brood_mother: Arc<Mutex<BroodMotherBase>>,
    spider: Arc<Mutex<spider::SpiderBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct BroodMotherBase {
    pub(crate) health: i64,
    pub(crate) eggs: i64,
}

impl BroodMother {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<BroodMotherInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: BroodMother = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// How much health this BroodMother has left. When it reaches 0, she dies and her owner loses.
    pub fn health(&self) -> i64 {
        self.inner().brood_mother.lock().unwrap().health.clone()
    }

    /// How many eggs the BroodMother has to spawn Spiderlings this turn.
    pub fn eggs(&self) -> i64 {
        self.inner().brood_mother.lock().unwrap().eggs.clone()
    }

    /// _Inherited from [`Spider`]_
    ///
    /// The Player that owns this Spider, and can command it.
    pub fn owner(&self) -> Player {
        self.inner().spider.lock().unwrap().owner.clone()
    }

    /// _Inherited from [`Spider`]_
    ///
    /// The Nest that this Spider is currently on. None when moving on a Web.
    pub fn nest(&self) -> Option<Nest> {
        self.inner().spider.lock().unwrap().nest.clone()
    }

    /// _Inherited from [`Spider`]_
    ///
    /// If this Spider is dead and has been removed from the game.
    pub fn is_dead(&self) -> bool {
        self.inner().spider.lock().unwrap().is_dead.clone()
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

    /// Consumes a Spiderling of the same owner to regain some eggs to spawn more Spiderlings.
    ///
    /// # Arguments
    ///
    /// - _spiderling_ - The Spiderling to consume. It must be on the same Nest as this
    /// BroodMother.
    ///
    /// # Returns
    ///
    /// True if the Spiderling was consumed. False otherwise.
    pub fn consume(
        &self,
        spiderling: &Spiderling,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            spiderling: &'a Spiderling,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            spiderling,
            _a: PhantomData,
        };
        self.context().run(&self.id, "consume", args)
    }

    /// Spawns a new Spiderling on the same Nest as this BroodMother, consuming an egg.
    ///
    /// # Arguments
    ///
    /// - _spiderling_type_ - The string name of the Spiderling class you want to Spawn. Must be
    /// 'Spitter', 'Weaver', or 'Cutter'.
    ///
    /// # Returns
    ///
    /// The newly spwaned Spiderling if successful. None otherwise.
    pub fn spawn(
        &self,
        spiderling_type: &str,
    )
        -> Result<Option<Spiderling>, Error>
    {
        struct Args<'a> {
            spiderling_type: &'a str,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            spiderling_type,
            _a: PhantomData,
        };
        self.context().run(&self.id, "spawn", args)
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
            x if x == TypeId::of::<BroodMother>() => Some(T::shallow(self.context.clone(), self.id.clone())),
            x if x == TypeId::of::<Spider>() => Some(T::shallow(self.context.clone(), self.id.clone())),
            x if x == TypeId::of::<GameObject>() => Some(T::shallow(self.context.clone(), self.id.clone())),
            _ => None,
        }
    }
}

impl ObjectInner for BroodMother {
    fn shallow(context: Weak<Context>, id: Str) -> BroodMother {
        BroodMother {
            context,
            id,
            inner: RefCell::new(None),
        }
    }
}

impl Object for BroodMother {}
