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
    Structure(StructureInner),
    Unit(UnitInner),
    Job(JobInner),
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
            AnyGameObject::Structure(obj) => Some(&obj.game_object),
            AnyGameObject::Unit(obj) => Some(&obj.game_object),
            AnyGameObject::Job(obj) => Some(&obj.game_object),
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
            AnyGameObject::Structure(_obj) => None,
            AnyGameObject::Unit(_obj) => None,
            AnyGameObject::Job(_obj) => None,
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
            AnyGameObject::Structure(_obj) => None,
            AnyGameObject::Unit(_obj) => None,
            AnyGameObject::Job(_obj) => None,
        }
    }

    pub fn as_tile(&self) -> &TileBase {
        self.try_as_tile().expect("unreachable: unable to cast to Tile")
    }

    pub fn try_as_structure(&self) -> Option< &StructureBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Tile(_obj) => None,
            AnyGameObject::Structure(obj) => Some(&obj.structure),
            AnyGameObject::Unit(_obj) => None,
            AnyGameObject::Job(_obj) => None,
        }
    }

    pub fn as_structure(&self) -> &StructureBase {
        self.try_as_structure().expect("unreachable: unable to cast to Structure")
    }

    pub fn try_as_unit(&self) -> Option< &UnitBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Tile(_obj) => None,
            AnyGameObject::Structure(_obj) => None,
            AnyGameObject::Unit(obj) => Some(&obj.unit),
            AnyGameObject::Job(_obj) => None,
        }
    }

    pub fn as_unit(&self) -> &UnitBase {
        self.try_as_unit().expect("unreachable: unable to cast to Unit")
    }

    pub fn try_as_job(&self) -> Option< &JobBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Tile(_obj) => None,
            AnyGameObject::Structure(_obj) => None,
            AnyGameObject::Unit(_obj) => None,
            AnyGameObject::Job(obj) => Some(&obj.job),
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
pub struct StructureInner {
    pub structure: StructureBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct UnitInner {
    pub unit: UnitBase,
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
    pub units: List<Unit>,
    pub cat: Unit,
    pub upkeep: i64,
    pub structures: List<Structure>,
    pub food: i64,
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
    pub structure: Option<Structure>,
    pub harvest_rate: i64,
    pub turns_to_harvest: i64,
    pub materials: i64,
    pub food: i64,
}

#[derive(Debug, Clone)]
pub struct StructureBase {
    pub type_: Str,
    pub tile: Option<Tile>,
    pub owner: Option<Player>,
    pub materials: i64,
    pub effect_radius: i64,
}

#[derive(Debug, Clone)]
pub struct UnitBase {
    pub owner: Option<Player>,
    pub tile: Option<Tile>,
    pub job: Job,
    pub moves: i64,
    pub energy: f64,
    pub squad: List<Unit>,
    pub acted: bool,
    pub food: i64,
    pub materials: i64,
    pub starving: bool,
    pub turns_to_die: i64,
    pub movement_target: Option<Tile>,
}

#[derive(Debug, Clone)]
pub struct JobBase {
    pub title: Str,
    pub moves: i64,
    pub action_cost: f64,
    pub regen_rate: f64,
    pub carry_limit: i64,
    pub upkeep: i64,
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
    pub structures: List<Structure>,
    pub jobs: List<Job>,
    pub cat_energy_mult: f64,
    pub starving_energy_mult: f64,
    pub monument_cost_mult: f64,
    pub harvest_cooldown: i64,
    pub turns_to_create_human: i64,
    pub turns_to_lower_harvest: i64,
    pub lower_harvest_amount: i64,
    pub turns_between_harvests: i64,
    pub neutral_materials: i64,
    pub wall_materials: i64,
    pub shelter_materials: i64,
    pub monument_materials: i64,
    pub starting_food: i64,
}
