#![allow(unused_imports, dead_code)]

use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Context {
    self_ref: Option<Weak<Mutex<Context>>>,
    game: Game,
}

impl Context {
    pub(crate) fn get_ref(&self) -> Weak<Mutex<Context>> {
        self.self_ref.clone().unwrap()
    }

    pub(crate) fn run<A, R>(&mut self, _caller: &str, _function_name: &str, _args: A) -> Result<R, Error> {
        unimplemented!()
    }
}

pub trait Object: ObjectInner {
}

pub trait ObjectInner: Sized {
    fn from_game_object(game_obj: &Arc<Mutex<AnyGameObject>>, context: &Weak<Mutex<Context>>) -> Option<Self>;
}

#[derive(Debug, Clone)]
pub enum AnyGameObject {
    GameObject(GameObjectInner),
    Player(PlayerInner),
    Tile(TileInner),
    Tower(TowerInner),
    Unit(UnitInner),
    UnitJob(UnitJobInner),
    TowerJob(TowerJobInner),
}

impl AnyGameObject {
    pub fn id(&self) -> Str {
        self.as_game_object().id.clone()
    }

    pub fn object_type(&self) -> Str {
        self.as_game_object().game_object_name.clone()
    }

    pub fn try_as_game_object(&self) -> Option< &GameObjectBase > {
        match self {
            AnyGameObject::GameObject(obj) => Some(&obj.game_object),
            AnyGameObject::Player(obj) => Some(&obj.game_object),
            AnyGameObject::Tile(obj) => Some(&obj.game_object),
            AnyGameObject::Tower(obj) => Some(&obj.game_object),
            AnyGameObject::Unit(obj) => Some(&obj.game_object),
            AnyGameObject::UnitJob(obj) => Some(&obj.game_object),
            AnyGameObject::TowerJob(obj) => Some(&obj.game_object),
        }
    }

    pub fn as_game_object(&self) -> &GameObjectBase {
        self.try_as_game_object().expect("unreachable: unable to cast to GameObject")
    }

    pub fn try_as_player(&self) -> Option< &PlayerBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(obj) => Some(&obj.player),
            AnyGameObject::Tile(_obj) => None,
            AnyGameObject::Tower(_obj) => None,
            AnyGameObject::Unit(_obj) => None,
            AnyGameObject::UnitJob(_obj) => None,
            AnyGameObject::TowerJob(_obj) => None,
        }
    }

    pub fn as_player(&self) -> &PlayerBase {
        self.try_as_player().expect("unreachable: unable to cast to Player")
    }

    pub fn try_as_tile(&self) -> Option< &TileBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Tile(obj) => Some(&obj.tile),
            AnyGameObject::Tower(_obj) => None,
            AnyGameObject::Unit(_obj) => None,
            AnyGameObject::UnitJob(_obj) => None,
            AnyGameObject::TowerJob(_obj) => None,
        }
    }

    pub fn as_tile(&self) -> &TileBase {
        self.try_as_tile().expect("unreachable: unable to cast to Tile")
    }

    pub fn try_as_tower(&self) -> Option< &TowerBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Tile(_obj) => None,
            AnyGameObject::Tower(obj) => Some(&obj.tower),
            AnyGameObject::Unit(_obj) => None,
            AnyGameObject::UnitJob(_obj) => None,
            AnyGameObject::TowerJob(_obj) => None,
        }
    }

    pub fn as_tower(&self) -> &TowerBase {
        self.try_as_tower().expect("unreachable: unable to cast to Tower")
    }

    pub fn try_as_unit(&self) -> Option< &UnitBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Tile(_obj) => None,
            AnyGameObject::Tower(_obj) => None,
            AnyGameObject::Unit(obj) => Some(&obj.unit),
            AnyGameObject::UnitJob(_obj) => None,
            AnyGameObject::TowerJob(_obj) => None,
        }
    }

    pub fn as_unit(&self) -> &UnitBase {
        self.try_as_unit().expect("unreachable: unable to cast to Unit")
    }

    pub fn try_as_unit_job(&self) -> Option< &UnitJobBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Tile(_obj) => None,
            AnyGameObject::Tower(_obj) => None,
            AnyGameObject::Unit(_obj) => None,
            AnyGameObject::UnitJob(obj) => Some(&obj.unit_job),
            AnyGameObject::TowerJob(_obj) => None,
        }
    }

    pub fn as_unit_job(&self) -> &UnitJobBase {
        self.try_as_unit_job().expect("unreachable: unable to cast to UnitJob")
    }

    pub fn try_as_tower_job(&self) -> Option< &TowerJobBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Tile(_obj) => None,
            AnyGameObject::Tower(_obj) => None,
            AnyGameObject::Unit(_obj) => None,
            AnyGameObject::UnitJob(_obj) => None,
            AnyGameObject::TowerJob(obj) => Some(&obj.tower_job),
        }
    }

    pub fn as_tower_job(&self) -> &TowerJobBase {
        self.try_as_tower_job().expect("unreachable: unable to cast to TowerJob")
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
pub struct TowerInner {
    pub tower: TowerBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct UnitInner {
    pub unit: UnitBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct UnitJobInner {
    pub unit_job: UnitJobBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct TowerJobInner {
    pub tower_job: TowerJobBase,
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
    pub units: List<Unit>,
    pub towers: List<Tower>,
    pub gold: i64,
    pub mana: i64,
    pub health: i64,
    pub side: List<Tile>,
    pub home_base: List<Tile>,
}

#[derive(Debug, Clone)]
pub struct TileBase {
    pub x: i64,
    pub y: i64,
    pub tile_north: Option<Tile>,
    pub tile_east: Option<Tile>,
    pub tile_south: Option<Tile>,
    pub tile_west: Option<Tile>,
    pub unit: Option<Unit>,
    pub tower: Option<Tower>,
    pub is_wall: bool,
    pub is_path: bool,
    pub is_grass: bool,
    pub is_tower: bool,
    pub is_gold_mine: bool,
    pub is_island_gold_mine: bool,
    pub is_river: bool,
    pub is_castle: bool,
    pub is_worker_spawn: bool,
    pub is_unit_spawn: bool,
    pub corpses: i64,
    pub num_zombies: i64,
    pub num_ghouls: i64,
    pub num_hounds: i64,
    pub owner: Option<Player>,
}

#[derive(Debug, Clone)]
pub struct TowerBase {
    pub owner: Option<Player>,
    pub tile: Tile,
    pub job: TowerJob,
    pub health: i64,
    pub attacked: bool,
    pub cooldown: i64,
}

#[derive(Debug, Clone)]
pub struct UnitBase {
    pub owner: Option<Player>,
    pub tile: Option<Tile>,
    pub job: UnitJob,
    pub health: i64,
    pub acted: bool,
    pub moves: i64,
}

#[derive(Debug, Clone)]
pub struct UnitJobBase {
    pub title: Str,
    pub per_tile: i64,
    pub health: i64,
    pub moves: i64,
    pub damage: i64,
    pub gold_cost: i64,
    pub mana_cost: i64,
    pub range: i64,
}

#[derive(Debug, Clone)]
pub struct TowerJobBase {
    pub title: Str,
    pub health: i64,
    pub range: i64,
    pub all_units: bool,
    pub damage: i64,
    pub gold_cost: i64,
    pub turns_between_attacks: i64,
    pub mana_cost: i64,
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
    pub units: List<Unit>,
    pub unit_jobs: List<UnitJob>,
    pub towers: List<Tower>,
    pub tower_jobs: List<TowerJob>,
    pub river_phase: i64,
    pub gold_income_per_unit: i64,
    pub island_income_per_unit: i64,
    pub mana_income_per_unit: i64,
}
