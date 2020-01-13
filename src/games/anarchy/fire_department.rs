#![allow(dead_code, unused_imports)]

use std::sync::{Arc, Mutex, Weak};
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;

use super::*;
use crate::types::*;
use crate::error::Error;

/// Can put out fires completely.
#[derive(Debug, Clone)]
pub struct FireDepartment {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<FireDepartmentInner>>,
}

#[derive(Debug, Clone)]
struct FireDepartmentInner {
    fire_department: Arc<Mutex<FireDepartmentBase>>,
    building: Arc<Mutex<building::BuildingBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct FireDepartmentBase {
    pub(crate) fire_extinguished: i64,
}

impl FireDepartment {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<FireDepartmentInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: FireDepartment = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The amount of fire removed from a building when bribed to extinguish a building.
    pub fn fire_extinguished(&self) -> i64 {
        self.inner().fire_department.lock().unwrap().fire_extinguished.clone()
    }

    /// _Inherited from Building_
    ///
    /// How much health this building currently has. When this reaches 0 the Building has been
    /// burned down.
    pub fn health(&self) -> i64 {
        self.inner().building.lock().unwrap().health.clone()
    }

    /// _Inherited from Building_
    ///
    /// The player that owns this building. If it burns down (health reaches 0) that player gets an
    /// additional bribe(s).
    pub fn owner(&self) -> Player {
        self.inner().building.lock().unwrap().owner.clone()
    }

    /// _Inherited from Building_
    ///
    /// True if this is the Headquarters of the owning player, false otherwise. Burning this down
    /// wins the game for the other Player.
    pub fn is_headquarters(&self) -> bool {
        self.inner().building.lock().unwrap().is_headquarters.clone()
    }

    /// _Inherited from Building_
    ///
    /// When true this building has already been bribed this turn and cannot be bribed again this
    /// turn.
    pub fn bribed(&self) -> bool {
        self.inner().building.lock().unwrap().bribed.clone()
    }

    /// _Inherited from Building_
    ///
    /// The location of the Building along the x-axis.
    pub fn x(&self) -> i64 {
        self.inner().building.lock().unwrap().x.clone()
    }

    /// _Inherited from Building_
    ///
    /// The location of the Building along the y-axis.
    pub fn y(&self) -> i64 {
        self.inner().building.lock().unwrap().y.clone()
    }

    /// _Inherited from Building_
    ///
    /// How much fire is currently burning the building, and thus how much damage it will take at
    /// the end of its owner's turn. 0 means no fire.
    pub fn fire(&self) -> i64 {
        self.inner().building.lock().unwrap().fire.clone()
    }

    /// _Inherited from Building_
    ///
    /// The Building directly to the north of this building, or None if not present.
    pub fn building_north(&self) -> Option<Building> {
        self.inner().building.lock().unwrap().building_north.clone()
    }

    /// _Inherited from Building_
    ///
    /// The Building directly to the east of this building, or None if not present.
    pub fn building_east(&self) -> Option<Building> {
        self.inner().building.lock().unwrap().building_east.clone()
    }

    /// _Inherited from Building_
    ///
    /// The Building directly to the south of this building, or None if not present.
    pub fn building_south(&self) -> Option<Building> {
        self.inner().building.lock().unwrap().building_south.clone()
    }

    /// _Inherited from Building_
    ///
    /// The Building directly to the west of this building, or None if not present.
    pub fn building_west(&self) -> Option<Building> {
        self.inner().building.lock().unwrap().building_west.clone()
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

    /// Bribes this FireDepartment to extinguish the some of the fire in a building.
    ///
    /// # Arguments
    ///
    /// - _building_ - The Building you want to extinguish.
    ///
    /// # Returns
    ///
    /// True if the bribe worked, false otherwise.
    pub fn extinguish(
        &self,
        building: &Building,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            building: &'a Building,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            building,
            _a: PhantomData,
        };
        self.context().run(&self.id, "extinguish", args)
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
