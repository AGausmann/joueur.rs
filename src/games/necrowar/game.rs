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
    pub(crate) units: List<Unit>,
    pub(crate) unit_jobs: List<UnitJob>,
    pub(crate) towers: List<Tower>,
    pub(crate) tower_jobs: List<TowerJob>,
    pub(crate) river_phase: i64,
    pub(crate) gold_income_per_unit: i64,
    pub(crate) island_income_per_unit: i64,
    pub(crate) mana_income_per_unit: i64,
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

    /// Every Unit in the game.
    pub fn units(&self) -> List<Unit> {
        self.inner().game.lock().unwrap().units.clone()
    }

    /// A list of every unit type / job.
    pub fn unit_jobs(&self) -> List<UnitJob> {
        self.inner().game.lock().unwrap().unit_jobs.clone()
    }

    /// Every Tower in the game.
    pub fn towers(&self) -> List<Tower> {
        self.inner().game.lock().unwrap().towers.clone()
    }

    /// A list of every tower type / job.
    pub fn tower_jobs(&self) -> List<TowerJob> {
        self.inner().game.lock().unwrap().tower_jobs.clone()
    }

    /// The amount of turns it takes between the river changing phases.
    pub fn river_phase(&self) -> i64 {
        self.inner().game.lock().unwrap().river_phase.clone()
    }

    /// The amount of gold income per turn per unit in a mine.
    pub fn gold_income_per_unit(&self) -> i64 {
        self.inner().game.lock().unwrap().gold_income_per_unit.clone()
    }

    /// The amount of gold income per turn per unit in the island mine.
    pub fn island_income_per_unit(&self) -> i64 {
        self.inner().game.lock().unwrap().island_income_per_unit.clone()
    }

    /// The Amount of gold income per turn per unit fishing on the river side.
    pub fn mana_income_per_unit(&self) -> i64 {
        self.inner().game.lock().unwrap().mana_income_per_unit.clone()
    }
}
