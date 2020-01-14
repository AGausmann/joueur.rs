#![allow(dead_code, unused_imports)]

use std::any::TypeId;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A Tile in the game that makes up the 2D map grid.
#[derive(Debug, Clone)]
pub struct Tile {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<TileInner>>,
}

#[derive(Debug, Clone)]
struct TileInner {
    tile: Arc<Mutex<TileBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct TileBase {
    pub(crate) x: i64,
    pub(crate) y: i64,
    pub(crate) tile_north: Option<Tile>,
    pub(crate) tile_east: Option<Tile>,
    pub(crate) tile_south: Option<Tile>,
    pub(crate) tile_west: Option<Tile>,
    pub(crate) cowboy: Option<Cowboy>,
    pub(crate) furnishing: Option<Furnishing>,
    pub(crate) is_balcony: bool,
    pub(crate) has_hazard: bool,
    pub(crate) bottle: Option<Bottle>,
    pub(crate) young_gun: Option<YoungGun>,
}

impl Tile {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<TileInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Tile = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The x (horizontal) position of this Tile.
    pub fn x(&self) -> i64 {
        self.inner().tile.lock().unwrap().x.clone()
    }

    /// The y (vertical) position of this Tile.
    pub fn y(&self) -> i64 {
        self.inner().tile.lock().unwrap().y.clone()
    }

    /// The Tile to the 'North' of this one (x, y-1). None if out of bounds of the map.
    pub fn tile_north(&self) -> Option<Tile> {
        self.inner().tile.lock().unwrap().tile_north.clone()
    }

    /// The Tile to the 'East' of this one (x+1, y). None if out of bounds of the map.
    pub fn tile_east(&self) -> Option<Tile> {
        self.inner().tile.lock().unwrap().tile_east.clone()
    }

    /// The Tile to the 'South' of this one (x, y+1). None if out of bounds of the map.
    pub fn tile_south(&self) -> Option<Tile> {
        self.inner().tile.lock().unwrap().tile_south.clone()
    }

    /// The Tile to the 'West' of this one (x-1, y). None if out of bounds of the map.
    pub fn tile_west(&self) -> Option<Tile> {
        self.inner().tile.lock().unwrap().tile_west.clone()
    }

    /// The Cowboy that is on this Tile, None otherwise.
    pub fn cowboy(&self) -> Option<Cowboy> {
        self.inner().tile.lock().unwrap().cowboy.clone()
    }

    /// The furnishing that is on this Tile, None otherwise.
    pub fn furnishing(&self) -> Option<Furnishing> {
        self.inner().tile.lock().unwrap().furnishing.clone()
    }

    /// If this Tile is a balcony of the Saloon that YoungGuns walk around on, and can never be
    /// pathed through by Cowboys.
    pub fn is_balcony(&self) -> bool {
        self.inner().tile.lock().unwrap().is_balcony.clone()
    }

    /// If this Tile is pathable, but has a hazard that damages Cowboys that path through it.
    pub fn has_hazard(&self) -> bool {
        self.inner().tile.lock().unwrap().has_hazard.clone()
    }

    /// The beer Bottle currently flying over this Tile, None otherwise.
    pub fn bottle(&self) -> Option<Bottle> {
        self.inner().tile.lock().unwrap().bottle.clone()
    }

    /// The YoungGun on this tile, None otherwise.
    pub fn young_gun(&self) -> Option<YoungGun> {
        self.inner().tile.lock().unwrap().young_gun.clone()
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
            x if x == TypeId::of::<Tile>() => Some(T::shallow(self.context.clone(), self.id.clone())),
            x if x == TypeId::of::<GameObject>() => Some(T::shallow(self.context.clone(), self.id.clone())),
            _ => None,
        }
    }
}

impl ObjectInner for Tile {
    fn shallow(context: Weak<Context>, id: Str) -> Tile {
        Tile {
            context,
            id,
            inner: RefCell::new(None),
        }
    }
}

impl Object for Tile {}
