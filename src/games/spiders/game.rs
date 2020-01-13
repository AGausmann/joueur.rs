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
    pub(crate) nests: List<Nest>,
    pub(crate) webs: List<Web>,
    pub(crate) movement_speed: i64,
    pub(crate) weave_speed: i64,
    pub(crate) cut_speed: i64,
    pub(crate) spit_speed: i64,
    pub(crate) weave_power: i64,
    pub(crate) initial_web_strength: i64,
    pub(crate) max_web_strength: i64,
    pub(crate) eggs_scalar: f64,
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

    /// Every Nest in the game.
    pub fn nests(&self) -> List<Nest> {
        self.inner().game.lock().unwrap().nests.clone()
    }

    /// Every Web in the game.
    pub fn webs(&self) -> List<Web> {
        self.inner().game.lock().unwrap().webs.clone()
    }

    /// The speed at which Spiderlings move on Webs.
    pub fn movement_speed(&self) -> i64 {
        self.inner().game.lock().unwrap().movement_speed.clone()
    }

    /// The speed at which Weavers work to do strengthens and weakens on Webs.
    pub fn weave_speed(&self) -> i64 {
        self.inner().game.lock().unwrap().weave_speed.clone()
    }

    /// The speed at which Cutters work to do cut Webs.
    pub fn cut_speed(&self) -> i64 {
        self.inner().game.lock().unwrap().cut_speed.clone()
    }

    /// The speed at which Spitters work to spit new Webs.
    pub fn spit_speed(&self) -> i64 {
        self.inner().game.lock().unwrap().spit_speed.clone()
    }

    /// How much web strength is added or removed from Webs when they are weaved.
    pub fn weave_power(&self) -> i64 {
        self.inner().game.lock().unwrap().weave_power.clone()
    }

    /// The starting strength for Webs.
    pub fn initial_web_strength(&self) -> i64 {
        self.inner().game.lock().unwrap().initial_web_strength.clone()
    }

    /// The maximum strength a web can be strengthened to.
    pub fn max_web_strength(&self) -> i64 {
        self.inner().game.lock().unwrap().max_web_strength.clone()
    }

    /// Constant used to calculate how many eggs BroodMothers get on their owner's turns.
    pub fn eggs_scalar(&self) -> f64 {
        self.inner().game.lock().unwrap().eggs_scalar.clone()
    }

    pub fn try_cast<T>(&self) -> Option<T> {
        self.context().try_get_obj(&self.id)
    }

    pub fn cast<T>(&self) -> Option<T> {
        self.context().get_obj(&self.id)
    }
}
