#![allow(dead_code, unused_imports)]

use std::sync::{Arc, Mutex, Weak};
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;

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
    pub(crate) unit: Option<Unit>,
    pub(crate) tower: Option<Tower>,
    pub(crate) is_wall: bool,
    pub(crate) is_path: bool,
    pub(crate) is_grass: bool,
    pub(crate) is_tower: bool,
    pub(crate) is_gold_mine: bool,
    pub(crate) is_island_gold_mine: bool,
    pub(crate) is_river: bool,
    pub(crate) is_castle: bool,
    pub(crate) is_worker_spawn: bool,
    pub(crate) is_unit_spawn: bool,
    pub(crate) corpses: i64,
    pub(crate) num_zombies: i64,
    pub(crate) num_ghouls: i64,
    pub(crate) num_hounds: i64,
    pub(crate) owner: Option<Player>,
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

    /// The Tower on this Tile if present, otherwise None.
    pub fn tower(&self) -> Option<Tower> {
        self.inner().tile.lock().unwrap().tower.clone()
    }

    /// Whether or not the tile can be moved on by workers.
    pub fn is_wall(&self) -> bool {
        self.inner().tile.lock().unwrap().is_wall.clone()
    }

    /// Whether or not the tile is considered a path or not (Units can walk on paths).
    pub fn is_path(&self) -> bool {
        self.inner().tile.lock().unwrap().is_path.clone()
    }

    /// Whether or not the tile is considered grass or not (Workers can walk on grass).
    pub fn is_grass(&self) -> bool {
        self.inner().tile.lock().unwrap().is_grass.clone()
    }

    /// Whether or not the tile is considered a tower or not.
    pub fn is_tower(&self) -> bool {
        self.inner().tile.lock().unwrap().is_tower.clone()
    }

    /// Whether or not the tile is considered to be a gold mine or not.
    pub fn is_gold_mine(&self) -> bool {
        self.inner().tile.lock().unwrap().is_gold_mine.clone()
    }

    /// Whether or not the tile is considered to be the island gold mine or not.
    pub fn is_island_gold_mine(&self) -> bool {
        self.inner().tile.lock().unwrap().is_island_gold_mine.clone()
    }

    /// Whether or not the tile is considered a river or not.
    pub fn is_river(&self) -> bool {
        self.inner().tile.lock().unwrap().is_river.clone()
    }

    /// Whether or not the tile is a castle tile.
    pub fn is_castle(&self) -> bool {
        self.inner().tile.lock().unwrap().is_castle.clone()
    }

    /// Whether or not the tile is the worker spawn.
    pub fn is_worker_spawn(&self) -> bool {
        self.inner().tile.lock().unwrap().is_worker_spawn.clone()
    }

    /// Whether or not the tile is the unit spawn.
    pub fn is_unit_spawn(&self) -> bool {
        self.inner().tile.lock().unwrap().is_unit_spawn.clone()
    }

    /// The amount of corpses on this tile.
    pub fn corpses(&self) -> i64 {
        self.inner().tile.lock().unwrap().corpses.clone()
    }

    /// The amount of Zombies on this tile.
    pub fn num_zombies(&self) -> i64 {
        self.inner().tile.lock().unwrap().num_zombies.clone()
    }

    /// The amount of Ghouls on this tile.
    pub fn num_ghouls(&self) -> i64 {
        self.inner().tile.lock().unwrap().num_ghouls.clone()
    }

    /// The amount of Hounds on this tile.
    pub fn num_hounds(&self) -> i64 {
        self.inner().tile.lock().unwrap().num_hounds.clone()
    }

    /// Which player owns this tile, only applies to grass tiles for workers, None otherwise.
    pub fn owner(&self) -> Option<Player> {
        self.inner().tile.lock().unwrap().owner.clone()
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

    /// Resurrect the corpses on this tile into Zombies.
    ///
    /// # Arguments
    ///
    /// - _num_ - Number of zombies to resurrect.
    ///
    /// # Returns
    ///
    /// True if successful res, false otherwise.
    pub fn res(
        &self,
        num: i64,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            num: i64,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            num,
            _a: PhantomData,
        };
        self.context().run(&self.id, "res", args)
    }

    /// Spawns a fighting unit on the correct tile.
    ///
    /// # Arguments
    ///
    /// - _title_ - The title of the desired unit type.
    ///
    /// # Returns
    ///
    /// True if successfully spawned, false otherwise.
    pub fn spawn_unit(
        &self,
        title: &str,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            title: &'a str,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            title,
            _a: PhantomData,
        };
        self.context().run(&self.id, "spawnUnit", args)
    }

    /// Spawns a worker on the correct tile.
    ///
    /// # Returns
    ///
    /// True if successfully spawned, false otherwise.
    pub fn spawn_worker(
        &self,
    )
        -> Result<bool, Error>
    {
        struct Args<'a> {
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            _a: PhantomData,
        };
        self.context().run(&self.id, "spawnWorker", args)
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
