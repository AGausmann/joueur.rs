#![allow(dead_code, unused_imports)]

use std::sync::{Arc, Mutex, Weak};
use std::cell::{RefCell, RefMut};

use super::*;
use crate::types::*;

/// A machine in the game. Used to refine ore.
#[derive(Debug, Clone)]
pub struct Machine {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<MachineInner>>,
}

#[derive(Debug, Clone)]
struct MachineInner {
    machine: Arc<Mutex<MachineBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct MachineBase {
    pub(crate) tile: Tile,
    pub(crate) worked: i64,
    pub(crate) ore_type: Str,
    pub(crate) refine_time: i64,
    pub(crate) refine_input: i64,
    pub(crate) refine_output: i64,
}

impl Machine {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<MachineInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Machine = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The Tile this Machine is on.
    pub fn tile(&self) -> Tile {
        self.inner().machine.lock().unwrap().tile.clone()
    }

    /// Tracks how many times this machine has been worked. (0 to refineTime).
    pub fn worked(&self) -> i64 {
        self.inner().machine.lock().unwrap().worked.clone()
    }

    /// What type of ore the machine takes it. Also determines the type of material it outputs.
    /// (redium or blueium).
    pub fn ore_type(&self) -> Str {
        self.inner().machine.lock().unwrap().ore_type.clone()
    }

    /// The number of times this machine needs to be worked to refine ore.
    pub fn refine_time(&self) -> i64 {
        self.inner().machine.lock().unwrap().refine_time.clone()
    }

    /// The amount of ore that needs to be inputted into the machine for it to be worked.
    pub fn refine_input(&self) -> i64 {
        self.inner().machine.lock().unwrap().refine_input.clone()
    }

    /// The amount of refined ore that is returned after the machine has been fully worked.
    pub fn refine_output(&self) -> i64 {
        self.inner().machine.lock().unwrap().refine_output.clone()
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
        _message: &str,
    )
    {
        unimplemented!()
    }

    pub fn try_cast<T>(&self) -> Option<T> {
        self.context().try_get_obj(&self.id)
    }

    pub fn cast<T>(&self) -> Option<T> {
        self.context().get_obj(&self.id)
    }
}
