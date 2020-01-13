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
    pub(crate) structures: List<Structure>,
    pub(crate) jobs: List<Job>,
    pub(crate) cat_energy_mult: f64,
    pub(crate) starving_energy_mult: f64,
    pub(crate) monument_cost_mult: f64,
    pub(crate) harvest_cooldown: i64,
    pub(crate) turns_to_create_human: i64,
    pub(crate) turns_to_lower_harvest: i64,
    pub(crate) lower_harvest_amount: i64,
    pub(crate) turns_between_harvests: i64,
    pub(crate) neutral_materials: i64,
    pub(crate) wall_materials: i64,
    pub(crate) shelter_materials: i64,
    pub(crate) monument_materials: i64,
    pub(crate) starting_food: i64,
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

    /// Every Unit in the game.
    pub fn units(&self) -> List<Unit> {
        self.inner().game.lock().unwrap().units.clone()
    }

    /// Every Structure in the game.
    pub fn structures(&self) -> List<Structure> {
        self.inner().game.lock().unwrap().structures.clone()
    }

    /// All the Jobs that Units can have in the game.
    pub fn jobs(&self) -> List<Job> {
        self.inner().game.lock().unwrap().jobs.clone()
    }

    /// The multiplier for the amount of energy regenerated when resting in a shelter with the cat
    /// overlord.
    pub fn cat_energy_mult(&self) -> f64 {
        self.inner().game.lock().unwrap().cat_energy_mult.clone()
    }

    /// The multiplier for the amount of energy regenerated when resting while starving.
    pub fn starving_energy_mult(&self) -> f64 {
        self.inner().game.lock().unwrap().starving_energy_mult.clone()
    }

    /// The multiplier for the cost of actions when performing them in range of a monument. Does
    /// not effect pickup cost.
    pub fn monument_cost_mult(&self) -> f64 {
        self.inner().game.lock().unwrap().monument_cost_mult.clone()
    }

    /// The amount of turns it takes for a Tile that was just harvested to grow food again.
    pub fn harvest_cooldown(&self) -> i64 {
        self.inner().game.lock().unwrap().harvest_cooldown.clone()
    }

    /// The number of turns between fresh humans being spawned on the road.
    pub fn turns_to_create_human(&self) -> i64 {
        self.inner().game.lock().unwrap().turns_to_create_human.clone()
    }

    /// The number of turns before the harvest rate is lowered (length of each season basically).
    pub fn turns_to_lower_harvest(&self) -> i64 {
        self.inner().game.lock().unwrap().turns_to_lower_harvest.clone()
    }

    /// The amount that the harvest rate is lowered each season.
    pub fn lower_harvest_amount(&self) -> i64 {
        self.inner().game.lock().unwrap().lower_harvest_amount.clone()
    }

    /// After a food tile is harvested, the number of turns before it can be harvested again.
    pub fn turns_between_harvests(&self) -> i64 {
        self.inner().game.lock().unwrap().turns_between_harvests.clone()
    }

    /// The number of materials in a neutral Structure.
    pub fn neutral_materials(&self) -> i64 {
        self.inner().game.lock().unwrap().neutral_materials.clone()
    }

    /// The number of materials in a wall.
    pub fn wall_materials(&self) -> i64 {
        self.inner().game.lock().unwrap().wall_materials.clone()
    }

    /// The number of materials in a shelter.
    pub fn shelter_materials(&self) -> i64 {
        self.inner().game.lock().unwrap().shelter_materials.clone()
    }

    /// The number of materials in a monument.
    pub fn monument_materials(&self) -> i64 {
        self.inner().game.lock().unwrap().monument_materials.clone()
    }

    /// The amount of food Players start with.
    pub fn starting_food(&self) -> i64 {
        self.inner().game.lock().unwrap().starting_food.clone()
    }

    pub fn try_cast<T>(&self) -> Option<T> {
        self.context().try_get_obj(&self.id)
    }

    pub fn cast<T>(&self) -> Option<T> {
        self.context().get_obj(&self.id)
    }
}
