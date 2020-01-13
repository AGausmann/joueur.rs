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
    pub(crate) jobs: List<Job>,
    pub(crate) machines: List<Machine>,
    pub(crate) refined_value: i64,
    pub(crate) spawn_time: i64,
    pub(crate) manager_cap: i64,
    pub(crate) intern_cap: i64,
    pub(crate) physicist_cap: i64,
    pub(crate) stun_time: i64,
    pub(crate) time_immune: i64,
    pub(crate) material_spawn: i64,
    pub(crate) regenerate_rate: f64,
    pub(crate) victory_amount: i64,
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

    /// A list of all jobs. first item is intern, second is physicists, and third is manager.
    pub fn jobs(&self) -> List<Job> {
        self.inner().game.lock().unwrap().jobs.clone()
    }

    /// Every Machine in the game.
    pub fn machines(&self) -> List<Machine> {
        self.inner().game.lock().unwrap().machines.clone()
    }

    /// The amount of victory points added when a refined ore is consumed by the generator.
    pub fn refined_value(&self) -> i64 {
        self.inner().game.lock().unwrap().refined_value.clone()
    }

    /// The amount of turns it takes a unit to spawn.
    pub fn spawn_time(&self) -> i64 {
        self.inner().game.lock().unwrap().spawn_time.clone()
    }

    /// The maximum number of managers a player can have.
    pub fn manager_cap(&self) -> i64 {
        self.inner().game.lock().unwrap().manager_cap.clone()
    }

    /// The maximum number of interns a player can have.
    pub fn intern_cap(&self) -> i64 {
        self.inner().game.lock().unwrap().intern_cap.clone()
    }

    /// The maximum number of physicists a player can have.
    pub fn physicist_cap(&self) -> i64 {
        self.inner().game.lock().unwrap().physicist_cap.clone()
    }

    /// The amount of turns a unit cannot do anything when stunned.
    pub fn stun_time(&self) -> i64 {
        self.inner().game.lock().unwrap().stun_time.clone()
    }

    /// The number turns a unit is immune to being stunned.
    pub fn time_immune(&self) -> i64 {
        self.inner().game.lock().unwrap().time_immune.clone()
    }

    /// The number of materials that spawn per spawn cycle.
    pub fn material_spawn(&self) -> i64 {
        self.inner().game.lock().unwrap().material_spawn.clone()
    }

    /// The percent of max HP regained when a unit end their turn on a tile owned by their player.
    pub fn regenerate_rate(&self) -> f64 {
        self.inner().game.lock().unwrap().regenerate_rate.clone()
    }

    /// The amount of combined heat and pressure that you need to win.
    pub fn victory_amount(&self) -> i64 {
        self.inner().game.lock().unwrap().victory_amount.clone()
    }
}
