#![allow(dead_code, unused_imports)]

use std::any::TypeId;
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;
use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// Holds top-level game state and settings for the current game.
#[derive(Debug, Clone)]
pub struct Game {
    context: Weak<Context>,
    id: Str,
    inner: RefCell<Option<GameInner>>,
}

#[derive(Debug, Clone)]
struct GameInner {
    game: Arc<Mutex<GameBase>>,
}

#[derive(Debug)]
pub(crate) struct GameBase {
    pub(crate) game_objects: Map<Str, GameObject>,
    pub(crate) players: List<Player>,
    pub(crate) session: Str,
    pub(crate) current_player: Player,
    pub(crate) current_turn: i64,
    pub(crate) max_turns: i64,
    pub(crate) time_added_per_turn: i64,
    pub(crate) map_width: i64,
    pub(crate) map_height: i64,
    pub(crate) tiles: List<Tile>,
    pub(crate) beavers: List<Beaver>,
    pub(crate) spawner: List<Spawner>,
    pub(crate) spawner_types: List<Str>,
    pub(crate) jobs: List<Job>,
    pub(crate) free_beavers_count: i64,
    pub(crate) lodges_to_win: i64,
    pub(crate) lodge_cost_constant: f64,
    pub(crate) spawner_harvest_constant: f64,
}

impl Game {
    fn context(&self) -> Arc<Context> {
        self.context.upgrade().expect("context dropped before end of game")
    }

    fn inner(&self) -> RefMut<GameInner> {
        let inner = self.inner.borrow_mut();
        RefMut::map(inner, |cache| {
            if let Some(resolved) = cache {
                resolved
            } else {
                panic!("game is unresolved?");
            }
        })
    }


    /// A mapping of every game object's ID to the actual game object. Primarily used by the server
    /// and client to easily refer to the game objects via ID.
    pub fn game_objects(&self) -> Map<Str, GameObject> {
        self.inner().game.lock().unwrap().game_objects.clone()
    }

    /// List of all the players in the game.
    pub fn players(&self) -> List<Player> {
        self.inner().game.lock().unwrap().players.clone()
    }

    /// A unique identifier for the game instance that is being played.
    pub fn session(&self) -> Str {
        self.inner().game.lock().unwrap().session.clone()
    }

    /// The player whose turn it is currently. That player can send commands. Other players cannot.
    pub fn current_player(&self) -> Player {
        self.inner().game.lock().unwrap().current_player.clone()
    }

    /// The current turn number, starting at 0 for the first player's turn.
    pub fn current_turn(&self) -> i64 {
        self.inner().game.lock().unwrap().current_turn.clone()
    }

    /// The maximum number of turns before the game will automatically end.
    pub fn max_turns(&self) -> i64 {
        self.inner().game.lock().unwrap().max_turns.clone()
    }

    /// The amount of time (in nano-seconds) added after each player performs a turn.
    pub fn time_added_per_turn(&self) -> i64 {
        self.inner().game.lock().unwrap().time_added_per_turn.clone()
    }

    /// The number of Tiles in the map along the x (horizontal) axis.
    pub fn map_width(&self) -> i64 {
        self.inner().game.lock().unwrap().map_width.clone()
    }

    /// The number of Tiles in the map along the y (vertical) axis.
    pub fn map_height(&self) -> i64 {
        self.inner().game.lock().unwrap().map_height.clone()
    }

    /// All the tiles in the map, stored in Row-major order. Use `x + y * mapWidth` to access the
    /// correct index.
    pub fn tiles(&self) -> List<Tile> {
        self.inner().game.lock().unwrap().tiles.clone()
    }

    /// Every Beaver in the game.
    pub fn beavers(&self) -> List<Beaver> {
        self.inner().game.lock().unwrap().beavers.clone()
    }

    /// Every Spawner in the game.
    pub fn spawner(&self) -> List<Spawner> {
        self.inner().game.lock().unwrap().spawner.clone()
    }

    /// All the types of Spawners in the game.
    pub fn spawner_types(&self) -> List<Str> {
        self.inner().game.lock().unwrap().spawner_types.clone()
    }

    /// All the Jobs that Beavers can have in the game.
    pub fn jobs(&self) -> List<Job> {
        self.inner().game.lock().unwrap().jobs.clone()
    }

    /// When a Player has less Beavers than this number, then recruiting other Beavers is free.
    pub fn free_beavers_count(&self) -> i64 {
        self.inner().game.lock().unwrap().free_beavers_count.clone()
    }

    /// How many lodges must be owned by a Player at once to win the game.
    pub fn lodges_to_win(&self) -> i64 {
        self.inner().game.lock().unwrap().lodges_to_win.clone()
    }

    /// Constant number used to calculate what it costs to spawn a new lodge.
    pub fn lodge_cost_constant(&self) -> f64 {
        self.inner().game.lock().unwrap().lodge_cost_constant.clone()
    }

    /// Constant number used to calculate how many branches/food Beavers harvest from Spawners.
    pub fn spawner_harvest_constant(&self) -> f64 {
        self.inner().game.lock().unwrap().spawner_harvest_constant.clone()
    }
}
