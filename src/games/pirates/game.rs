#![allow(dead_code, unused_imports)]

use std::sync::{Arc, Mutex, Weak};
use std::cell::{RefCell, RefMut};

use super::*;
use crate::types::*;

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
    pub(crate) ports: List<Port>,
    pub(crate) crew_cost: i64,
    pub(crate) ship_cost: i64,
    pub(crate) crew_damage: i64,
    pub(crate) ship_damage: i64,
    pub(crate) crew_health: i64,
    pub(crate) ship_health: i64,
    pub(crate) crew_range: f64,
    pub(crate) ship_range: f64,
    pub(crate) crew_moves: i64,
    pub(crate) ship_moves: i64,
    pub(crate) rest_range: f64,
    pub(crate) heal_factor: f64,
    pub(crate) bury_interest_rate: f64,
    pub(crate) merchant_interest_rate: f64,
    pub(crate) min_interest_distance: f64,
    pub(crate) merchant_gold_rate: f64,
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
                let obj: Game = self.context().get_obj(&self.id);
                *cache = obj.inner.borrow().clone();
                cache.as_mut().unwrap()
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

    /// Every Unit in the game. Merchant units have targetPort set to a port.
    pub fn units(&self) -> List<Unit> {
        self.inner().game.lock().unwrap().units.clone()
    }

    /// Every Port in the game. Merchant ports have owner set to None.
    pub fn ports(&self) -> List<Port> {
        self.inner().game.lock().unwrap().ports.clone()
    }

    /// How much gold it costs to construct a single crew.
    pub fn crew_cost(&self) -> i64 {
        self.inner().game.lock().unwrap().crew_cost.clone()
    }

    /// How much gold it costs to construct a ship.
    pub fn ship_cost(&self) -> i64 {
        self.inner().game.lock().unwrap().ship_cost.clone()
    }

    /// How much damage crew deal to each other.
    pub fn crew_damage(&self) -> i64 {
        self.inner().game.lock().unwrap().crew_damage.clone()
    }

    /// How much damage ships deal to ships and ports.
    pub fn ship_damage(&self) -> i64 {
        self.inner().game.lock().unwrap().ship_damage.clone()
    }

    /// The maximum amount of health a crew member can have.
    pub fn crew_health(&self) -> i64 {
        self.inner().game.lock().unwrap().crew_health.clone()
    }

    /// The maximum amount of health a ship can have.
    pub fn ship_health(&self) -> i64 {
        self.inner().game.lock().unwrap().ship_health.clone()
    }

    /// A crew's attack range. Range is circular.
    pub fn crew_range(&self) -> f64 {
        self.inner().game.lock().unwrap().crew_range.clone()
    }

    /// A ship's attack range. Range is circular.
    pub fn ship_range(&self) -> f64 {
        self.inner().game.lock().unwrap().ship_range.clone()
    }

    /// The number of moves Units with only crew are given each turn.
    pub fn crew_moves(&self) -> i64 {
        self.inner().game.lock().unwrap().crew_moves.clone()
    }

    /// The number of moves Units with ships are given each turn.
    pub fn ship_moves(&self) -> i64 {
        self.inner().game.lock().unwrap().ship_moves.clone()
    }

    /// How far a Unit can be from a Port to rest. Range is circular.
    pub fn rest_range(&self) -> f64 {
        self.inner().game.lock().unwrap().rest_range.clone()
    }

    /// How much health a Unit recovers when they rest.
    pub fn heal_factor(&self) -> f64 {
        self.inner().game.lock().unwrap().heal_factor.clone()
    }

    /// The rate buried gold increases each turn.
    pub fn bury_interest_rate(&self) -> f64 {
        self.inner().game.lock().unwrap().bury_interest_rate.clone()
    }

    /// When a merchant ship spawns, the amount of additional gold it has relative to the Port's
    /// investment.
    pub fn merchant_interest_rate(&self) -> f64 {
        self.inner().game.lock().unwrap().merchant_interest_rate.clone()
    }

    /// The Euclidean distance buried gold must be from the Player's Port to accumulate interest.
    pub fn min_interest_distance(&self) -> f64 {
        self.inner().game.lock().unwrap().min_interest_distance.clone()
    }

    /// How much gold merchant Ports get each turn.
    pub fn merchant_gold_rate(&self) -> f64 {
        self.inner().game.lock().unwrap().merchant_gold_rate.clone()
    }

    pub fn try_cast<T>(&self) -> Option<T> {
        self.context().try_get_obj(&self.id)
    }

    pub fn cast<T>(&self) -> Option<T> {
        self.context().get_obj(&self.id)
    }
}
