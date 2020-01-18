#![allow(unused_imports, dead_code)]

use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Context {
    game: GameBase,
}

impl Context {
    pub fn run<A, R>(&mut self, _caller: &str, _function_name: &str, _args: A) -> Result<R, Error> {
        unimplemented!()
    }
}

pub trait Object: ObjectInner {
}

pub trait ObjectInner: Sized {
    fn from_game_object(game_obj: &Arc<Mutex<GameObject>>, context: &Weak<Mutex<Context>>) -> Option<Self>;
}

#[derive(Debug, Clone)]
pub enum GameObject {
    GameObject(GameObjectInner),
    Player(PlayerInner),
    Tile(TileInner),
    Spawner(SpawnerInner),
    Beaver(BeaverInner),
    Job(JobInner),
}

impl GameObject {
    pub fn id(&self) -> Str {
        self.as_game_object().id.clone()
    }

    pub fn object_type(&self) -> Str {
        self.as_game_object().game_object_name.clone()
    }

    pub fn try_as_game_object(&self) -> Option< &GameObjectBase > {
        match self {
            GameObject::GameObject(obj) => Some(&obj.game_object),
            GameObject::Player(obj) => Some(&obj.game_object),
            GameObject::Tile(obj) => Some(&obj.game_object),
            GameObject::Spawner(obj) => Some(&obj.game_object),
            GameObject::Beaver(obj) => Some(&obj.game_object),
            GameObject::Job(obj) => Some(&obj.game_object),
        }
    }

    pub fn as_game_object(&self) -> &GameObjectBase {
        self.try_as_game_object().expect("unreachable: unable to cast to GameObject")
    }

    pub fn try_as_player(&self) -> Option< &PlayerBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(obj) => Some(&obj.player),
            GameObject::Tile(_obj) => None,
            GameObject::Spawner(_obj) => None,
            GameObject::Beaver(_obj) => None,
            GameObject::Job(_obj) => None,
        }
    }

    pub fn as_player(&self) -> &PlayerBase {
        self.try_as_player().expect("unreachable: unable to cast to Player")
    }

    pub fn try_as_tile(&self) -> Option< &TileBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(obj) => Some(&obj.tile),
            GameObject::Spawner(_obj) => None,
            GameObject::Beaver(_obj) => None,
            GameObject::Job(_obj) => None,
        }
    }

    pub fn as_tile(&self) -> &TileBase {
        self.try_as_tile().expect("unreachable: unable to cast to Tile")
    }

    pub fn try_as_spawner(&self) -> Option< &SpawnerBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Spawner(obj) => Some(&obj.spawner),
            GameObject::Beaver(_obj) => None,
            GameObject::Job(_obj) => None,
        }
    }

    pub fn as_spawner(&self) -> &SpawnerBase {
        self.try_as_spawner().expect("unreachable: unable to cast to Spawner")
    }

    pub fn try_as_beaver(&self) -> Option< &BeaverBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Spawner(_obj) => None,
            GameObject::Beaver(obj) => Some(&obj.beaver),
            GameObject::Job(_obj) => None,
        }
    }

    pub fn as_beaver(&self) -> &BeaverBase {
        self.try_as_beaver().expect("unreachable: unable to cast to Beaver")
    }

    pub fn try_as_job(&self) -> Option< &JobBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Spawner(_obj) => None,
            GameObject::Beaver(_obj) => None,
            GameObject::Job(obj) => Some(&obj.job),
        }
    }

    pub fn as_job(&self) -> &JobBase {
        self.try_as_job().expect("unreachable: unable to cast to Job")
    }
}

#[derive(Debug, Clone)]
pub struct GameObjectInner {
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct PlayerInner {
    pub player: PlayerBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct TileInner {
    pub tile: TileBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct SpawnerInner {
    pub spawner: SpawnerBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct BeaverInner {
    pub beaver: BeaverBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct JobInner {
    pub job: JobBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct GameObjectBase {
    pub id: Str,
    pub game_object_name: Str,
    pub logs: List<Str>,
}

#[derive(Debug, Clone)]
pub struct PlayerBase {
    pub name: Str,
    pub client_type: Str,
    pub won: bool,
    pub lost: bool,
    pub reason_won: Str,
    pub reason_lost: Str,
    pub time_remaining: f64,
    pub opponent: Player,
    pub beavers: List<Beaver>,
    pub lodges: List<Tile>,
    pub branches_to_build_lodge: i64,
}

#[derive(Debug, Clone)]
pub struct TileBase {
    pub x: i64,
    pub y: i64,
    pub tile_north: Option<Tile>,
    pub tile_east: Option<Tile>,
    pub tile_south: Option<Tile>,
    pub tile_west: Option<Tile>,
    pub type_: Str,
    pub flow_direction: Str,
    pub beaver: Option<Beaver>,
    pub spawner: Option<Spawner>,
    pub lodge_owner: Option<Player>,
    pub branches: i64,
    pub food: i64,
}

#[derive(Debug, Clone)]
pub struct SpawnerBase {
    pub type_: Str,
    pub health: i64,
    pub tile: Tile,
    pub has_been_harvested: bool,
}

#[derive(Debug, Clone)]
pub struct BeaverBase {
    pub moves: i64,
    pub owner: Player,
    pub actions: i64,
    pub tile: Option<Tile>,
    pub health: i64,
    pub turns_distracted: i64,
    pub branches: i64,
    pub food: i64,
    pub job: Job,
    pub recruited: bool,
}

#[derive(Debug, Clone)]
pub struct JobBase {
    pub title: Str,
    pub health: i64,
    pub moves: i64,
    pub actions: i64,
    pub damage: i64,
    pub chopping: i64,
    pub munching: i64,
    pub distraction_power: i64,
    pub carry_limit: i64,
    pub cost: i64,
}

#[derive(Debug, Clone)]
pub struct GameBase {
    pub game_objects: Map<Str, GameObject>,
    pub players: List<Player>,
    pub session: Str,
    pub current_player: Player,
    pub current_turn: i64,
    pub max_turns: i64,
    pub time_added_per_turn: i64,
    pub map_width: i64,
    pub map_height: i64,
    pub tiles: List<Tile>,
    pub beavers: List<Beaver>,
    pub spawner: List<Spawner>,
    pub spawner_types: List<Str>,
    pub jobs: List<Job>,
    pub free_beavers_count: i64,
    pub lodges_to_win: i64,
    pub lodge_cost_constant: f64,
    pub spawner_harvest_constant: f64,
}
