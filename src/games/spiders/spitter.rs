#![allow(dead_code, unused_imports)]

use std::sync::{Arc, Mutex, Weak};
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;

use super::*;
use crate::types::*;
use crate::error::Error;

/// A Spiderling that creates and spits new Webs from the Nest it is on to another Nest, connecting
/// them.
#[derive(Debug, Clone)]
pub struct Spitter {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<SpitterInner>>,
}

#[derive(Debug, Clone)]
struct SpitterInner {
    spitter: Arc<Mutex<SpitterBase>>,
    spiderling: Arc<Mutex<spiderling::SpiderlingBase>>,
    spider: Arc<Mutex<spider::SpiderBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct SpitterBase {
    pub(crate) spitting_web_to_nest: Option<Nest>,
}

impl Spitter {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<SpitterInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Spitter = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The Nest that this Spitter is creating a Web to spit at, thus connecting them. None if not
    /// spitting.
    pub fn spitting_web_to_nest(&self) -> Option<Nest> {
        self.inner().spitter.lock().unwrap().spitting_web_to_nest.clone()
    }

    /// _Inherited from Spiderling_
    ///
    /// When empty string this Spiderling is not busy, and can act. Otherwise a string representing
    /// what it is busy with, e.g. 'Moving', 'Attacking'.
    pub fn busy(&self) -> Str {
        self.inner().spiderling.lock().unwrap().busy.clone()
    }

    /// _Inherited from Spiderling_
    ///
    /// How much work needs to be done for this Spiderling to finish being busy. See docs for the
    /// Work forumla.
    pub fn work_remaining(&self) -> f64 {
        self.inner().spiderling.lock().unwrap().work_remaining.clone()
    }

    /// _Inherited from Spiderling_
    ///
    /// The number of Spiderlings busy with the same work this Spiderling is doing, speeding up the
    /// task.
    pub fn number_of_coworkers(&self) -> i64 {
        self.inner().spiderling.lock().unwrap().number_of_coworkers.clone()
    }

    /// _Inherited from Spiderling_
    ///
    /// The Web this Spiderling is using to move. None if it is not moving.
    pub fn moving_on_web(&self) -> Option<Web> {
        self.inner().spiderling.lock().unwrap().moving_on_web.clone()
    }

    /// _Inherited from Spiderling_
    ///
    /// The Nest this Spiderling is moving to. None if it is not moving.
    pub fn moving_to_nest(&self) -> Option<Nest> {
        self.inner().spiderling.lock().unwrap().moving_to_nest.clone()
    }

    /// _Inherited from Spider_
    ///
    /// The Player that owns this Spider, and can command it.
    pub fn owner(&self) -> Player {
        self.inner().spider.lock().unwrap().owner.clone()
    }

    /// _Inherited from Spider_
    ///
    /// The Nest that this Spider is currently on. None when moving on a Web.
    pub fn nest(&self) -> Option<Nest> {
        self.inner().spider.lock().unwrap().nest.clone()
    }

    /// _Inherited from Spider_
    ///
    /// If this Spider is dead and has been removed from the game.
    pub fn is_dead(&self) -> bool {
        self.inner().spider.lock().unwrap().is_dead.clone()
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

    /// Creates and spits a new Web from the Nest the Spitter is on to another Nest, connecting
    /// them.
    ///
    /// # Arguments
    ///
    /// - _nest_ - The Nest you want to spit a Web to, thus connecting that Nest and the one the
    /// Spitter is on.
    ///
    /// # Returns
    ///
    /// True if the spit was successful, false otherwise.
    pub fn spit(
        &self,
        nest: &Nest,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            nest: &'a Nest,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            nest,
            _a: PhantomData,
        };
        self.context().run(&self.id, "spit", args)
    }

    /// _Inherited from Spiderling_
    ///
    /// Starts moving the Spiderling across a Web to another Nest.
    ///
    /// # Arguments
    ///
    /// - _web_ - The Web you want to move across to the other Nest.
    ///
    /// # Returns
    ///
    /// True if the move was successful, false otherwise.
    pub fn move_(
        &self,
        web: &Web,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            web: &'a Web,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            web,
            _a: PhantomData,
        };
        self.context().run(&self.id, "move", args)
    }

    /// _Inherited from Spiderling_
    ///
    /// Attacks another Spiderling
    ///
    /// # Arguments
    ///
    /// - _spiderling_ - The Spiderling to attack.
    ///
    /// # Returns
    ///
    /// True if the attack was successful, false otherwise.
    pub fn attack(
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
        self.context().run(&self.id, "attack", args)
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

    pub fn try_cast<T>(&self) -> Option<T> {
        self.context().try_get_obj(&self.id)
    }

    pub fn cast<T>(&self) -> Option<T> {
        self.context().get_obj(&self.id)
    }
}
