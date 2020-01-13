#![allow(dead_code, unused_imports)]

use std::sync::{Arc, Mutex, Weak};
use std::cell::{RefCell, RefMut};

use super::*;
use crate::types::*;

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
    pub(crate) unit: Option<Unit>,
    pub(crate) port: Option<Port>,
    pub(crate) type_: Str,
    pub(crate) gold: i64,
    pub(crate) decoration: bool,
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

    /// The Unit on this Tile if present, otherwise None.
    pub fn unit(&self) -> Option<Unit> {
        self.inner().tile.lock().unwrap().unit.clone()
    }

    /// The Port on this Tile if present, otherwise None.
    pub fn port(&self) -> Option<Port> {
        self.inner().tile.lock().unwrap().port.clone()
    }

    /// The type of Tile this is ('water' or 'land').
    pub fn type_(&self) -> Str {
        self.inner().tile.lock().unwrap().type_.clone()
    }

    /// The amount of gold buried on this tile.
    pub fn gold(&self) -> i64 {
        self.inner().tile.lock().unwrap().gold.clone()
    }

    /// (Visualizer only) Whether this tile is deep sea or grassy. This has no effect on gameplay,
    /// but feel free to use it if you want.
    pub fn decoration(&self) -> bool {
        self.inner().tile.lock().unwrap().decoration.clone()
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
