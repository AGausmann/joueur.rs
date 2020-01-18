#![allow(unused_imports, dead_code)]

use super::*;
use crate::types::*;
use crate::error::Error;

#[derive(Debug, Clone)]
pub(crate) struct Context {
    game: GameBase,
}

impl Context {
    pub(crate) fn run<A, R>(&mut self, _caller: &str, _function_name: &str, _args: A) -> Result<R, Error> {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum GameObject {
    GameObject(GameObjectInner),
    Player(PlayerInner),
    Tile(TileInner),
    Port(PortInner),
    Unit(UnitInner),
}

impl GameObject {
    pub(crate) fn id(&self) -> Str {
        self.as_game_object().id.clone()
    }

    pub(crate) fn object_type(&self) -> Str {
        self.as_game_object().game_object_name.clone()
    }

    pub(crate) fn try_as_game_object(&self) -> Option< &GameObjectBase > {
        match self {
            GameObject::GameObject(obj) => Some(&obj.game_object),
            GameObject::Player(obj) => Some(&obj.game_object),
            GameObject::Tile(obj) => Some(&obj.game_object),
            GameObject::Port(obj) => Some(&obj.game_object),
            GameObject::Unit(obj) => Some(&obj.game_object),
        }
    }

    pub(crate) fn as_game_object(&self) -> &GameObjectBase {
        self.try_as_game_object().expect("unreachable: unable to cast to GameObject")
    }

    pub(crate) fn try_as_player(&self) -> Option< &PlayerBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(obj) => Some(&obj.player),
            GameObject::Tile(_obj) => None,
            GameObject::Port(_obj) => None,
            GameObject::Unit(_obj) => None,
        }
    }

    pub(crate) fn as_player(&self) -> &PlayerBase {
        self.try_as_player().expect("unreachable: unable to cast to Player")
    }

    pub(crate) fn try_as_tile(&self) -> Option< &TileBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(obj) => Some(&obj.tile),
            GameObject::Port(_obj) => None,
            GameObject::Unit(_obj) => None,
        }
    }

    pub(crate) fn as_tile(&self) -> &TileBase {
        self.try_as_tile().expect("unreachable: unable to cast to Tile")
    }

    pub(crate) fn try_as_port(&self) -> Option< &PortBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Port(obj) => Some(&obj.port),
            GameObject::Unit(_obj) => None,
        }
    }

    pub(crate) fn as_port(&self) -> &PortBase {
        self.try_as_port().expect("unreachable: unable to cast to Port")
    }

    pub(crate) fn try_as_unit(&self) -> Option< &UnitBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Port(_obj) => None,
            GameObject::Unit(obj) => Some(&obj.unit),
        }
    }

    pub(crate) fn as_unit(&self) -> &UnitBase {
        self.try_as_unit().expect("unreachable: unable to cast to Unit")
    }
}

#[derive(Debug, Clone)]
pub(crate) struct GameObjectInner {
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct PlayerInner {
    pub(crate) player: PlayerBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct TileInner {
    pub(crate) tile: TileBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct PortInner {
    pub(crate) port: PortBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct UnitInner {
    pub(crate) unit: UnitBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct GameObjectBase {
    pub(crate) id: Str,
    pub(crate) game_object_name: Str,
    pub(crate) logs: List<Str>,
}

#[derive(Debug, Clone)]
pub(crate) struct PlayerBase {
    pub(crate) name: Str,
    pub(crate) client_type: Str,
    pub(crate) won: bool,
    pub(crate) lost: bool,
    pub(crate) reason_won: Str,
    pub(crate) reason_lost: Str,
    pub(crate) time_remaining: f64,
    pub(crate) opponent: Player,
    pub(crate) units: List<Unit>,
    pub(crate) infamy: i64,
    pub(crate) gold: i64,
    pub(crate) port: Port,
}

#[derive(Debug, Clone)]
pub(crate) struct TileBase {
    pub(crate) x: i64,
    pub(crate) y: i64,
    pub(crate) tile_north: Option<Tile>,
    pub(crate) tile_east: Option<Tile>,
    pub(crate) tile_south: Option<Tile>,
    pub(crate) tile_west: Option<Tile>,
    pub(crate) unit: Option<Unit>,
    pub(crate) port: Option<Port>,
    pub(crate) type_: Str,
    pub(crate) gold: i64,
    pub(crate) decoration: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct PortBase {
    pub(crate) tile: Tile,
    pub(crate) owner: Option<Player>,
    pub(crate) gold: i64,
    pub(crate) investment: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct UnitBase {
    pub(crate) owner: Option<Player>,
    pub(crate) tile: Option<Tile>,
    pub(crate) ship_health: i64,
    pub(crate) crew: i64,
    pub(crate) crew_health: i64,
    pub(crate) gold: i64,
    pub(crate) acted: bool,
    pub(crate) moves: i64,
    pub(crate) path: List<Tile>,
    pub(crate) target_port: Option<Port>,
    pub(crate) stun_turns: i64,
}

#[derive(Debug, Clone)]
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