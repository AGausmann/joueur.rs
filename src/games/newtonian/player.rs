#![allow(dead_code, unused_imports)]

use std::any::TypeId;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A player in this game. Every AI controls one player.
#[derive(Debug, Clone)]
pub struct Player {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<PlayerInner>>,
}

#[derive(Debug, Clone)]
struct PlayerInner {
    player: Arc<Mutex<PlayerBase>>,
    game_object: Arc<Mutex<game_object::GameObjectBase>>,
}

#[derive(Debug)]
pub(crate) struct PlayerBase {
    pub(crate) name: Str,
    pub(crate) client_type: Str,
    pub(crate) won: bool,
    pub(crate) lost: bool,
    pub(crate) reason_won: Str,
    pub(crate) reason_lost: Str,
    pub(crate) time_remaining: f64,
    pub(crate) opponent: Player,
    pub(crate) units: List<Unit>,
    pub(crate) heat: i64,
    pub(crate) pressure: i64,
    pub(crate) intern_spawn: i64,
    pub(crate) physicist_spawn: i64,
    pub(crate) manager_spawn: i64,
    pub(crate) spawn_tiles: List<Tile>,
    pub(crate) generator_tiles: List<Tile>,
}

impl Player {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<PlayerInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                let obj: Player = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
            }
        })
    }


    /// The name of the player.
    pub fn name(&self) -> Str {
        self.inner().player.lock().unwrap().name.clone()
    }

    /// What type of client this is, e.g. 'Python', 'JavaScript', or some other language. For
    /// potential data mining purposes.
    pub fn client_type(&self) -> Str {
        self.inner().player.lock().unwrap().client_type.clone()
    }

    /// If the player won the game or not.
    pub fn won(&self) -> bool {
        self.inner().player.lock().unwrap().won.clone()
    }

    /// If the player lost the game or not.
    pub fn lost(&self) -> bool {
        self.inner().player.lock().unwrap().lost.clone()
    }

    /// The reason why the player won the game.
    pub fn reason_won(&self) -> Str {
        self.inner().player.lock().unwrap().reason_won.clone()
    }

    /// The reason why the player lost the game.
    pub fn reason_lost(&self) -> Str {
        self.inner().player.lock().unwrap().reason_lost.clone()
    }

    /// The amount of time (in ns) remaining for this AI to send commands.
    pub fn time_remaining(&self) -> f64 {
        self.inner().player.lock().unwrap().time_remaining.clone()
    }

    /// This player's opponent in the game.
    pub fn opponent(&self) -> Player {
        self.inner().player.lock().unwrap().opponent.clone()
    }

    /// Every Unit owned by this Player.
    pub fn units(&self) -> List<Unit> {
        self.inner().player.lock().unwrap().units.clone()
    }

    /// The amount of heat this Player has.
    pub fn heat(&self) -> i64 {
        self.inner().player.lock().unwrap().heat.clone()
    }

    /// The amount of pressure this Player has.
    pub fn pressure(&self) -> i64 {
        self.inner().player.lock().unwrap().pressure.clone()
    }

    /// The time left till a intern spawns. (0 to spawnTime).
    pub fn intern_spawn(&self) -> i64 {
        self.inner().player.lock().unwrap().intern_spawn.clone()
    }

    /// The time left till a physicist spawns. (0 to spawnTime).
    pub fn physicist_spawn(&self) -> i64 {
        self.inner().player.lock().unwrap().physicist_spawn.clone()
    }

    /// The time left till a manager spawns. (0 to spawnTime).
    pub fn manager_spawn(&self) -> i64 {
        self.inner().player.lock().unwrap().manager_spawn.clone()
    }

    /// All the tiles this Player's units can spawn on. (listed from the outer edges inward, from
    /// top to bottom).
    pub fn spawn_tiles(&self) -> List<Tile> {
        self.inner().player.lock().unwrap().spawn_tiles.clone()
    }

    /// Every generator Tile owned by this Player. (listed from the outer edges inward, from top to
    /// bottom).
    pub fn generator_tiles(&self) -> List<Tile> {
        self.inner().player.lock().unwrap().generator_tiles.clone()
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
}

impl ObjectInner for Player {
    fn to_bases(&self) -> Bases {
        let inner = self.inner();
        Bases {
            context: Some(self.context.clone()),
            id: Some(self.id.clone()),
            player: Some(Arc::clone(&inner.player)),
            game_object: Some(Arc::clone(&inner.game_object)),
            ..Default::default()
        }
    }

    fn from_bases(bases: Bases) -> Option<Self> {
        let inner = PlayerInner {
            player: bases.player?,
            game_object: bases.game_object?,
        };

        Some(Player {
            context: bases.context?,
            id: bases.id?,
            inner: RefCell::new(Some(inner)),
        })
    }
}

impl Object for Player {}
