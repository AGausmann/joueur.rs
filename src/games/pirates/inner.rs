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
    Port(PortInner),
    Unit(UnitInner),
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
            AnyGameObject::Port(obj) => Some(&obj.game_object),
            AnyGameObject::Unit(obj) => Some(&obj.game_object),
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
            AnyGameObject::Port(_obj) => None,
            AnyGameObject::Unit(_obj) => None,
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
            AnyGameObject::Port(_obj) => None,
            AnyGameObject::Unit(_obj) => None,
        }
    }

    pub fn as_tile(&self) -> &TileBase {
        self.try_as_tile().expect("unreachable: unable to cast to Tile")
    }

    pub fn try_as_port(&self) -> Option< &PortBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Tile(_obj) => None,
            AnyGameObject::Port(obj) => Some(&obj.port),
            AnyGameObject::Unit(_obj) => None,
        }
    }

    pub fn as_port(&self) -> &PortBase {
        self.try_as_port().expect("unreachable: unable to cast to Port")
    }

    pub fn try_as_unit(&self) -> Option< &UnitBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Tile(_obj) => None,
            AnyGameObject::Port(_obj) => None,
            AnyGameObject::Unit(obj) => Some(&obj.unit),
        }
    }

    pub fn as_unit(&self) -> &UnitBase {
        self.try_as_unit().expect("unreachable: unable to cast to Unit")
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
pub struct PortInner {
    pub port: PortBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct UnitInner {
    pub unit: UnitBase,
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
    pub infamy: i64,
    pub gold: i64,
    pub port: Port,
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
    pub port: Option<Port>,
    pub type_: Str,
    pub gold: i64,
    pub decoration: bool,
}

#[derive(Debug, Clone)]
pub struct PortBase {
    pub tile: Tile,
    pub owner: Option<Player>,
    pub gold: i64,
    pub investment: i64,
}

#[derive(Debug, Clone)]
pub struct UnitBase {
    pub owner: Option<Player>,
    pub tile: Option<Tile>,
    pub ship_health: i64,
    pub crew: i64,
    pub crew_health: i64,
    pub gold: i64,
    pub acted: bool,
    pub moves: i64,
    pub path: List<Tile>,
    pub target_port: Option<Port>,
    pub stun_turns: i64,
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
    pub ports: List<Port>,
    pub crew_cost: i64,
    pub ship_cost: i64,
    pub crew_damage: i64,
    pub ship_damage: i64,
    pub crew_health: i64,
    pub ship_health: i64,
    pub crew_range: f64,
    pub ship_range: f64,
    pub crew_moves: i64,
    pub ship_moves: i64,
    pub rest_range: f64,
    pub heal_factor: f64,
    pub bury_interest_rate: f64,
    pub merchant_interest_rate: f64,
    pub min_interest_distance: f64,
    pub merchant_gold_rate: f64,
}
