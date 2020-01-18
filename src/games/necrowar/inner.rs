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
    Tower(TowerInner),
    Unit(UnitInner),
    UnitJob(UnitJobInner),
    TowerJob(TowerJobInner),
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
            GameObject::Tower(obj) => Some(&obj.game_object),
            GameObject::Unit(obj) => Some(&obj.game_object),
            GameObject::UnitJob(obj) => Some(&obj.game_object),
            GameObject::TowerJob(obj) => Some(&obj.game_object),
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
            GameObject::Tower(_obj) => None,
            GameObject::Unit(_obj) => None,
            GameObject::UnitJob(_obj) => None,
            GameObject::TowerJob(_obj) => None,
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
            GameObject::Tower(_obj) => None,
            GameObject::Unit(_obj) => None,
            GameObject::UnitJob(_obj) => None,
            GameObject::TowerJob(_obj) => None,
        }
    }

    pub(crate) fn as_tile(&self) -> &TileBase {
        self.try_as_tile().expect("unreachable: unable to cast to Tile")
    }

    pub(crate) fn try_as_tower(&self) -> Option< &TowerBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Tower(obj) => Some(&obj.tower),
            GameObject::Unit(_obj) => None,
            GameObject::UnitJob(_obj) => None,
            GameObject::TowerJob(_obj) => None,
        }
    }

    pub(crate) fn as_tower(&self) -> &TowerBase {
        self.try_as_tower().expect("unreachable: unable to cast to Tower")
    }

    pub(crate) fn try_as_unit(&self) -> Option< &UnitBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Tower(_obj) => None,
            GameObject::Unit(obj) => Some(&obj.unit),
            GameObject::UnitJob(_obj) => None,
            GameObject::TowerJob(_obj) => None,
        }
    }

    pub(crate) fn as_unit(&self) -> &UnitBase {
        self.try_as_unit().expect("unreachable: unable to cast to Unit")
    }

    pub(crate) fn try_as_unit_job(&self) -> Option< &UnitJobBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Tower(_obj) => None,
            GameObject::Unit(_obj) => None,
            GameObject::UnitJob(obj) => Some(&obj.unit_job),
            GameObject::TowerJob(_obj) => None,
        }
    }

    pub(crate) fn as_unit_job(&self) -> &UnitJobBase {
        self.try_as_unit_job().expect("unreachable: unable to cast to UnitJob")
    }

    pub(crate) fn try_as_tower_job(&self) -> Option< &TowerJobBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Tower(_obj) => None,
            GameObject::Unit(_obj) => None,
            GameObject::UnitJob(_obj) => None,
            GameObject::TowerJob(obj) => Some(&obj.tower_job),
        }
    }

    pub(crate) fn as_tower_job(&self) -> &TowerJobBase {
        self.try_as_tower_job().expect("unreachable: unable to cast to TowerJob")
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
pub(crate) struct TowerInner {
    pub(crate) tower: TowerBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct UnitInner {
    pub(crate) unit: UnitBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct UnitJobInner {
    pub(crate) unit_job: UnitJobBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct TowerJobInner {
    pub(crate) tower_job: TowerJobBase,
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
    pub(crate) towers: List<Tower>,
    pub(crate) gold: i64,
    pub(crate) mana: i64,
    pub(crate) health: i64,
    pub(crate) side: List<Tile>,
    pub(crate) home_base: List<Tile>,
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
    pub(crate) tower: Option<Tower>,
    pub(crate) is_wall: bool,
    pub(crate) is_path: bool,
    pub(crate) is_grass: bool,
    pub(crate) is_tower: bool,
    pub(crate) is_gold_mine: bool,
    pub(crate) is_island_gold_mine: bool,
    pub(crate) is_river: bool,
    pub(crate) is_castle: bool,
    pub(crate) is_worker_spawn: bool,
    pub(crate) is_unit_spawn: bool,
    pub(crate) corpses: i64,
    pub(crate) num_zombies: i64,
    pub(crate) num_ghouls: i64,
    pub(crate) num_hounds: i64,
    pub(crate) owner: Option<Player>,
}

#[derive(Debug, Clone)]
pub(crate) struct TowerBase {
    pub(crate) owner: Option<Player>,
    pub(crate) tile: Tile,
    pub(crate) job: TowerJob,
    pub(crate) health: i64,
    pub(crate) attacked: bool,
    pub(crate) cooldown: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct UnitBase {
    pub(crate) owner: Option<Player>,
    pub(crate) tile: Option<Tile>,
    pub(crate) job: UnitJob,
    pub(crate) health: i64,
    pub(crate) acted: bool,
    pub(crate) moves: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct UnitJobBase {
    pub(crate) title: Str,
    pub(crate) per_tile: i64,
    pub(crate) health: i64,
    pub(crate) moves: i64,
    pub(crate) damage: i64,
    pub(crate) gold_cost: i64,
    pub(crate) mana_cost: i64,
    pub(crate) range: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct TowerJobBase {
    pub(crate) title: Str,
    pub(crate) health: i64,
    pub(crate) range: i64,
    pub(crate) all_units: bool,
    pub(crate) damage: i64,
    pub(crate) gold_cost: i64,
    pub(crate) turns_between_attacks: i64,
    pub(crate) mana_cost: i64,
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
    pub(crate) unit_jobs: List<UnitJob>,
    pub(crate) towers: List<Tower>,
    pub(crate) tower_jobs: List<TowerJob>,
    pub(crate) river_phase: i64,
    pub(crate) gold_income_per_unit: i64,
    pub(crate) island_income_per_unit: i64,
    pub(crate) mana_income_per_unit: i64,
}
