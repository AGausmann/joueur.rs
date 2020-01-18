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
    Machine(MachineInner),
    Unit(UnitInner),
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
            GameObject::Machine(obj) => Some(&obj.game_object),
            GameObject::Unit(obj) => Some(&obj.game_object),
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
            GameObject::Machine(_obj) => None,
            GameObject::Unit(_obj) => None,
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
            GameObject::Machine(_obj) => None,
            GameObject::Unit(_obj) => None,
            GameObject::Job(_obj) => None,
        }
    }

    pub fn as_tile(&self) -> &TileBase {
        self.try_as_tile().expect("unreachable: unable to cast to Tile")
    }

    pub fn try_as_machine(&self) -> Option< &MachineBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Machine(obj) => Some(&obj.machine),
            GameObject::Unit(_obj) => None,
            GameObject::Job(_obj) => None,
        }
    }

    pub fn as_machine(&self) -> &MachineBase {
        self.try_as_machine().expect("unreachable: unable to cast to Machine")
    }

    pub fn try_as_unit(&self) -> Option< &UnitBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Machine(_obj) => None,
            GameObject::Unit(obj) => Some(&obj.unit),
            GameObject::Job(_obj) => None,
        }
    }

    pub fn as_unit(&self) -> &UnitBase {
        self.try_as_unit().expect("unreachable: unable to cast to Unit")
    }

    pub fn try_as_job(&self) -> Option< &JobBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Machine(_obj) => None,
            GameObject::Unit(_obj) => None,
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
pub struct MachineInner {
    pub machine: MachineBase,
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
    pub heat: i64,
    pub pressure: i64,
    pub intern_spawn: i64,
    pub physicist_spawn: i64,
    pub manager_spawn: i64,
    pub spawn_tiles: List<Tile>,
    pub generator_tiles: List<Tile>,
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
    pub machine: Option<Machine>,
    pub is_wall: bool,
    pub type_: Str,
    pub direction: Str,
    pub owner: Option<Player>,
    pub redium_ore: i64,
    pub redium: i64,
    pub blueium_ore: i64,
    pub blueium: i64,
    pub decoration: i64,
}

#[derive(Debug, Clone)]
pub struct MachineBase {
    pub tile: Tile,
    pub worked: i64,
    pub ore_type: Str,
    pub refine_time: i64,
    pub refine_input: i64,
    pub refine_output: i64,
}

#[derive(Debug, Clone)]
pub struct UnitBase {
    pub owner: Option<Player>,
    pub tile: Option<Tile>,
    pub job: Job,
    pub health: i64,
    pub acted: bool,
    pub moves: i64,
    pub redium_ore: i64,
    pub redium: i64,
    pub blueium_ore: i64,
    pub blueium: i64,
    pub stun_time: i64,
    pub stun_immune: i64,
}

#[derive(Debug, Clone)]
pub struct JobBase {
    pub title: Str,
    pub health: i64,
    pub moves: i64,
    pub damage: i64,
    pub carry_limit: i64,
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
    pub jobs: List<Job>,
    pub machines: List<Machine>,
    pub refined_value: i64,
    pub spawn_time: i64,
    pub manager_cap: i64,
    pub intern_cap: i64,
    pub physicist_cap: i64,
    pub stun_time: i64,
    pub time_immune: i64,
    pub material_spawn: i64,
    pub regenerate_rate: f64,
    pub victory_amount: i64,
}
