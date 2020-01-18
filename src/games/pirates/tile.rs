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

    /// The Port on this Tile if present, otherwise None.
    pub fn port(&self) -> Option<Port> {
        self.inner.lock().unwrap()
            .as_tile()
            .port.clone()
    }

    /// The type of Tile this is ('water' or 'land').
    pub fn type_(&self) -> Str {
        self.inner.lock().unwrap()
            .as_tile()
            .type_.clone()
    }

    /// The amount of gold buried on this tile.
    pub fn gold(&self) -> i64 {
        self.inner.lock().unwrap()
            .as_tile()
            .gold.clone()
    }

    /// (Visualizer only) Whether this tile is deep sea or grassy. This has no effect on gameplay,
    /// but feel free to use it if you want.
    pub fn decoration(&self) -> bool {
        self.inner.lock().unwrap()
            .as_tile()
            .decoration.clone()
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
