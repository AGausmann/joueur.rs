#![allow(dead_code, unused_imports)]

use std::sync::{Arc, Mutex, Weak};
use std::cell::{RefCell, RefMut};
use std::marker::PhantomData;

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
    pub(crate) units: List<Unit>,
    pub(crate) projectiles: List<Projectile>,
    pub(crate) jobs: List<Job>,
    pub(crate) bodies: List<Body>,
    pub(crate) size_x: i64,
    pub(crate) size_y: i64,
    pub(crate) dash_distance: i64,
    pub(crate) dash_cost: i64,
    pub(crate) max_asteroid: i64,
    pub(crate) min_asteroid: i64,
    pub(crate) ore_rarity_genarium: f64,
    pub(crate) ore_rarity_rarium: f64,
    pub(crate) ore_rarity_legendarium: f64,
    pub(crate) genarium_value: f64,
    pub(crate) rarium_value: f64,
    pub(crate) legendarium_value: f64,
    pub(crate) mythicite_amount: f64,
    pub(crate) regenerate_rate: f64,
    pub(crate) planet_recharge_rate: i64,
    pub(crate) planet_energy_cap: i64,
    pub(crate) mining_speed: i64,
    pub(crate) projectile_speed: i64,
    pub(crate) projectile_radius: i64,
    pub(crate) ship_radius: i64,
    pub(crate) turns_to_orbit: i64,
    pub(crate) orbits_protected: i64,
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

    /// Every Unit in the game.
    pub fn units(&self) -> List<Unit> {
        self.inner().game.lock().unwrap().units.clone()
    }

    /// Every projectile in the game.
    pub fn projectiles(&self) -> List<Projectile> {
        self.inner().game.lock().unwrap().projectiles.clone()
    }

    /// A list of all jobs. first item is corvette, second is missileboat, third is martyr, fourth
    /// is transport, and fifth is miner.
    pub fn jobs(&self) -> List<Job> {
        self.inner().game.lock().unwrap().jobs.clone()
    }

    /// All the celestial bodies in the game. The first two are planets and the third is the sun.
    /// The fourth is the VP asteroid. Everything else is normal asteroids.
    pub fn bodies(&self) -> List<Body> {
        self.inner().game.lock().unwrap().bodies.clone()
    }

    /// The size of the map in the X direction.
    pub fn size_x(&self) -> i64 {
        self.inner().game.lock().unwrap().size_x.clone()
    }

    /// The size of the map in the Y direction.
    pub fn size_y(&self) -> i64 {
        self.inner().game.lock().unwrap().size_y.clone()
    }

    /// The distance traveled each turn by dashing.
    pub fn dash_distance(&self) -> i64 {
        self.inner().game.lock().unwrap().dash_distance.clone()
    }

    /// The cost of dashing.
    pub fn dash_cost(&self) -> i64 {
        self.inner().game.lock().unwrap().dash_cost.clone()
    }

    /// The highest amount of material, that can be in a asteroid.
    pub fn max_asteroid(&self) -> i64 {
        self.inner().game.lock().unwrap().max_asteroid.clone()
    }

    /// The smallest amount of material, that can be in a asteroid.
    pub fn min_asteroid(&self) -> i64 {
        self.inner().game.lock().unwrap().min_asteroid.clone()
    }

    /// The rarity modifier of the most common ore. This controls how much spawns.
    pub fn ore_rarity_genarium(&self) -> f64 {
        self.inner().game.lock().unwrap().ore_rarity_genarium.clone()
    }

    /// The rarity modifier of the second rarest ore. This controls how much spawns.
    pub fn ore_rarity_rarium(&self) -> f64 {
        self.inner().game.lock().unwrap().ore_rarity_rarium.clone()
    }

    /// The rarity modifier of the rarest ore. This controls how much spawns.
    pub fn ore_rarity_legendarium(&self) -> f64 {
        self.inner().game.lock().unwrap().ore_rarity_legendarium.clone()
    }

    /// The value of every unit of genarium.
    pub fn genarium_value(&self) -> f64 {
        self.inner().game.lock().unwrap().genarium_value.clone()
    }

    /// The value of every unit of rarium.
    pub fn rarium_value(&self) -> f64 {
        self.inner().game.lock().unwrap().rarium_value.clone()
    }

    /// The value of every unit of legendarium.
    pub fn legendarium_value(&self) -> f64 {
        self.inner().game.lock().unwrap().legendarium_value.clone()
    }

    /// The amount of mythicite that spawns at the start of the game.
    pub fn mythicite_amount(&self) -> f64 {
        self.inner().game.lock().unwrap().mythicite_amount.clone()
    }

    /// The regeneration rate of asteroids.
    pub fn regenerate_rate(&self) -> f64 {
        self.inner().game.lock().unwrap().regenerate_rate.clone()
    }

    /// The amount of energy the planets restore each round.
    pub fn planet_recharge_rate(&self) -> i64 {
        self.inner().game.lock().unwrap().planet_recharge_rate.clone()
    }

    /// The amount of energy a planet can hold at once.
    pub fn planet_energy_cap(&self) -> i64 {
        self.inner().game.lock().unwrap().planet_energy_cap.clone()
    }

    /// The rate at which miners grab minerals from asteroids.
    pub fn mining_speed(&self) -> i64 {
        self.inner().game.lock().unwrap().mining_speed.clone()
    }

    /// The amount of distance missiles travel through space.
    pub fn projectile_speed(&self) -> i64 {
        self.inner().game.lock().unwrap().projectile_speed.clone()
    }

    /// The standard size of ships.
    pub fn projectile_radius(&self) -> i64 {
        self.inner().game.lock().unwrap().projectile_radius.clone()
    }

    /// The standard size of ships.
    pub fn ship_radius(&self) -> i64 {
        self.inner().game.lock().unwrap().ship_radius.clone()
    }

    /// The number of turns it takes for a asteroid to orbit the sun. (Asteroids move after each
    /// players turn).
    pub fn turns_to_orbit(&self) -> i64 {
        self.inner().game.lock().unwrap().turns_to_orbit.clone()
    }

    /// The number of orbit updates you cannot mine the mithicite asteroid.
    pub fn orbits_protected(&self) -> i64 {
        self.inner().game.lock().unwrap().orbits_protected.clone()
    }

    pub fn try_cast<T>(&self) -> Option<T> {
        self.context().try_get_obj(&self.id)
    }

    pub fn cast<T>(&self) -> Option<T> {
        self.context().get_obj(&self.id)
    }
}
