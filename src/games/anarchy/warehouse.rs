#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A typical abandoned warehouse... that anarchists hang out in and can be bribed to burn down
/// Buildings.
#[derive(Debug, Clone)]
pub struct Warehouse {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::GameObject>>,
}

impl Warehouse {
    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }

    /// The amount of fire added to buildings when bribed to ignite a building. Headquarters add
    /// more fire than normal Warehouses.
    pub fn fire_added(&self) -> i64 {
        self.inner.lock().unwrap().as_warehouse()
            .fire_added.clone()
    }

    /// How exposed the anarchists in this warehouse are to PoliceDepartments. Raises when bribed
    /// to ignite buildings, and drops each turn if not bribed.
    pub fn exposure(&self) -> i64 {
        self.inner.lock().unwrap().as_warehouse()
            .exposure.clone()
    }

    /// _Inherited from [`Building`]_
    ///
    /// How much health this building currently has. When this reaches 0 the Building has been
    /// burned down.
    pub fn health(&self) -> i64 {
        self.inner.lock().unwrap().as_building()
            .health.clone()
    }

    /// _Inherited from [`Building`]_
    ///
    /// The player that owns this building. If it burns down (health reaches 0) that player gets an
    /// additional bribe(s).
    pub fn owner(&self) -> Player {
        self.inner.lock().unwrap().as_building()
            .owner.clone()
    }

    /// _Inherited from [`Building`]_
    ///
    /// True if this is the Headquarters of the owning player, false otherwise. Burning this down
    /// wins the game for the other Player.
    pub fn is_headquarters(&self) -> bool {
        self.inner.lock().unwrap().as_building()
            .is_headquarters.clone()
    }

    /// _Inherited from [`Building`]_
    ///
    /// When true this building has already been bribed this turn and cannot be bribed again this
    /// turn.
    pub fn bribed(&self) -> bool {
        self.inner.lock().unwrap().as_building()
            .bribed.clone()
    }

    /// _Inherited from [`Building`]_
    ///
    /// The location of the Building along the x-axis.
    pub fn x(&self) -> i64 {
        self.inner.lock().unwrap().as_building()
            .x.clone()
    }

    /// _Inherited from [`Building`]_
    ///
    /// The location of the Building along the y-axis.
    pub fn y(&self) -> i64 {
        self.inner.lock().unwrap().as_building()
            .y.clone()
    }

    /// _Inherited from [`Building`]_
    ///
    /// How much fire is currently burning the building, and thus how much damage it will take at
    /// the end of its owner's turn. 0 means no fire.
    pub fn fire(&self) -> i64 {
        self.inner.lock().unwrap().as_building()
            .fire.clone()
    }

    /// _Inherited from [`Building`]_
    ///
    /// The Building directly to the north of this building, or None if not present.
    pub fn building_north(&self) -> Option<Building> {
        self.inner.lock().unwrap().as_building()
            .building_north.clone()
    }

    /// _Inherited from [`Building`]_
    ///
    /// The Building directly to the east of this building, or None if not present.
    pub fn building_east(&self) -> Option<Building> {
        self.inner.lock().unwrap().as_building()
            .building_east.clone()
    }

    /// _Inherited from [`Building`]_
    ///
    /// The Building directly to the south of this building, or None if not present.
    pub fn building_south(&self) -> Option<Building> {
        self.inner.lock().unwrap().as_building()
            .building_south.clone()
    }

    /// _Inherited from [`Building`]_
    ///
    /// The Building directly to the west of this building, or None if not present.
    pub fn building_west(&self) -> Option<Building> {
        self.inner.lock().unwrap().as_building()
            .building_west.clone()
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

    /// Bribes the Warehouse to light a Building on fire. This adds this building's fireAdded to
    /// their fire, and then this building's exposure is increased based on the Manhatten distance
    /// between the two buildings.
    ///
    /// # Arguments
    ///
    /// - _building_ - The Building you want to light on fire.
    ///
    /// # Returns
    ///
    /// The exposure added to this Building's exposure. -1 is returned if there was an error.
    pub fn ignite(
        &self,
        building: &Building,
    )
        -> Result<i64, Error>
    {
        struct Args<'a> {
            building: &'a Building,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            building,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "ignite", args))
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

impl inner::ObjectInner for Warehouse {
    fn from_game_object(game_obj: &Arc<Mutex<inner::GameObject>>, context: &Weak<Mutex<inner::Context>>) -> Option<Self> {
        let handle = game_obj.lock().unwrap();
        if handle.try_as_warehouse().is_some() {
            Some(Warehouse {
                inner: Arc::clone(&game_obj),
                context: context.clone(),
            })
        } else {
            None
        }
    }
}
impl Object for Warehouse {}
