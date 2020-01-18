#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A Tile in the game that makes up the 2D map grid.
#[derive(Debug, Clone)]
pub struct Tile {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::AnyGameObject>>,
}

impl Tile {
    pub(crate) fn new(inner: Arc<Mutex<inner::AnyGameObject>>, context: Weak<Mutex<inner::Context>>) -> Tile {
        Tile { inner, context }
    }

    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }

    /// The x (horizontal) position of this Tile.
    pub fn x(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_tile()
            .x.clone()
    }

    /// The y (vertical) position of this Tile.
    pub fn y(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_tile()
            .y.clone()
    }

    /// The Tile to the 'North' of this one (x, y-1). None if out of bounds of the map.
    pub fn tile_north(&self) -> Option<Tile> {
        self.inner.lock().unwrap()
            .as_tile()
            .tile_north.clone()
    }

    /// The Tile to the 'East' of this one (x+1, y). None if out of bounds of the map.
    pub fn tile_east(&self) -> Option<Tile> {
        self.inner.lock().unwrap()
            .as_tile()
            .tile_east.clone()
    }

    /// The Tile to the 'South' of this one (x, y+1). None if out of bounds of the map.
    pub fn tile_south(&self) -> Option<Tile> {
        self.inner.lock().unwrap()
            .as_tile()
            .tile_south.clone()
    }

    /// The Tile to the 'West' of this one (x-1, y). None if out of bounds of the map.
    pub fn tile_west(&self) -> Option<Tile> {
        self.inner.lock().unwrap()
            .as_tile()
            .tile_west.clone()
    }

    /// The Unit on this Tile if present, otherwise None.
    pub fn unit(&self) -> Option<Unit> {
        self.inner.lock().unwrap()
            .as_tile()
            .unit.clone()
    }

    /// The Tower on this Tile if present, otherwise None.
    pub fn tower(&self) -> Option<Tower> {
        self.inner.lock().unwrap()
            .as_tile()
            .tower.clone()
    }

    /// Whether or not the tile can be moved on by workers.
    pub fn is_wall(&self) -> bool {
        self.inner.lock().unwrap()
            .as_tile()
            .is_wall.clone()
    }

    /// Whether or not the tile is considered a path or not (Units can walk on paths).
    pub fn is_path(&self) -> bool {
        self.inner.lock().unwrap()
            .as_tile()
            .is_path.clone()
    }

    /// Whether or not the tile is considered grass or not (Workers can walk on grass).
    pub fn is_grass(&self) -> bool {
        self.inner.lock().unwrap()
            .as_tile()
            .is_grass.clone()
    }

    /// Whether or not the tile is considered a tower or not.
    pub fn is_tower(&self) -> bool {
        self.inner.lock().unwrap()
            .as_tile()
            .is_tower.clone()
    }

    /// Whether or not the tile is considered to be a gold mine or not.
    pub fn is_gold_mine(&self) -> bool {
        self.inner.lock().unwrap()
            .as_tile()
            .is_gold_mine.clone()
    }

    /// Whether or not the tile is considered to be the island gold mine or not.
    pub fn is_island_gold_mine(&self) -> bool {
        self.inner.lock().unwrap()
            .as_tile()
            .is_island_gold_mine.clone()
    }

    /// Whether or not the tile is considered a river or not.
    pub fn is_river(&self) -> bool {
        self.inner.lock().unwrap()
            .as_tile()
            .is_river.clone()
    }

    /// Whether or not the tile is a castle tile.
    pub fn is_castle(&self) -> bool {
        self.inner.lock().unwrap()
            .as_tile()
            .is_castle.clone()
    }

    /// Whether or not the tile is the worker spawn.
    pub fn is_worker_spawn(&self) -> bool {
        self.inner.lock().unwrap()
            .as_tile()
            .is_worker_spawn.clone()
    }

    /// Whether or not the tile is the unit spawn.
    pub fn is_unit_spawn(&self) -> bool {
        self.inner.lock().unwrap()
            .as_tile()
            .is_unit_spawn.clone()
    }

    /// The amount of corpses on this tile.
    pub fn corpses(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_tile()
            .corpses.clone()
    }

    /// The amount of Zombies on this tile.
    pub fn num_zombies(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_tile()
            .num_zombies.clone()
    }

    /// The amount of Ghouls on this tile.
    pub fn num_ghouls(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_tile()
            .num_ghouls.clone()
    }

    /// The amount of Hounds on this tile.
    pub fn num_hounds(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_tile()
            .num_hounds.clone()
    }

    /// Which player owns this tile, only applies to grass tiles for workers, None otherwise.
    pub fn owner(&self) -> Option<Player> {
        self.inner.lock().unwrap()
            .as_tile()
            .owner.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// A unique id for each instance of a GameObject or a sub class. Used for client and server
    /// communication. Should never change value after being set.
    pub fn id(&self) -> Str {
        self.inner.lock().unwrap()
            .as_game_object()
            .id.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// String representing the top level Class that this game object is an instance of. Used for
    /// reflection to create new instances on clients, but exposed for convenience should AIs want
    /// this data.
    pub fn game_object_name(&self) -> Str {
        self.inner.lock().unwrap()
            .as_game_object()
            .game_object_name.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// Any strings logged will be stored here. Intended for debugging.
    pub fn logs(&self) -> List<Str> {
        self.inner.lock().unwrap()
            .as_game_object()
            .logs.clone()
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
        self.with_context(|cx| cx.run(&self.id(), "res", args))
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
        self.with_context(|cx| cx.run(&self.id(), "spawnUnit", args))
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
        self.with_context(|cx| cx.run(&self.id(), "spawnWorker", args))
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

impl inner::ObjectInner for Tile {
    fn from_game_object(game_obj: &Arc<Mutex<inner::AnyGameObject>>, context: &Weak<Mutex<inner::Context>>) -> Option<Self> {
        let handle = game_obj.lock().unwrap();
        if handle.try_as_tile().is_some() {
            Some(Tile {
                inner: Arc::clone(&game_obj),
                context: context.clone(),
            })
        } else {
            None
        }
    }
}
impl Object for Tile {}
