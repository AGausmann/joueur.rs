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
    Spawner(SpawnerInner),
    Beaver(BeaverInner),
    Job(JobInner),
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
            GameObject::Spawner(obj) => Some(&obj.game_object),
            GameObject::Beaver(obj) => Some(&obj.game_object),
            GameObject::Job(obj) => Some(&obj.game_object),
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
            GameObject::Spawner(_obj) => None,
            GameObject::Beaver(_obj) => None,
            GameObject::Job(_obj) => None,
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
            GameObject::Spawner(_obj) => None,
            GameObject::Beaver(_obj) => None,
            GameObject::Job(_obj) => None,
        }
    }

    pub(crate) fn as_tile(&self) -> &TileBase {
        self.try_as_tile().expect("unreachable: unable to cast to Tile")
    }

    pub(crate) fn try_as_spawner(&self) -> Option< &SpawnerBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Spawner(obj) => Some(&obj.spawner),
            GameObject::Beaver(_obj) => None,
            GameObject::Job(_obj) => None,
        }
    }

    pub(crate) fn as_spawner(&self) -> &SpawnerBase {
        self.try_as_spawner().expect("unreachable: unable to cast to Spawner")
    }

    pub(crate) fn try_as_beaver(&self) -> Option< &BeaverBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Spawner(_obj) => None,
            GameObject::Beaver(obj) => Some(&obj.beaver),
            GameObject::Job(_obj) => None,
        }
    }

    pub(crate) fn as_beaver(&self) -> &BeaverBase {
        self.try_as_beaver().expect("unreachable: unable to cast to Beaver")
    }

    pub(crate) fn try_as_job(&self) -> Option< &JobBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Spawner(_obj) => None,
            GameObject::Beaver(_obj) => None,
            GameObject::Job(obj) => Some(&obj.job),
        }
    }

    pub(crate) fn as_job(&self) -> &JobBase {
        self.try_as_job().expect("unreachable: unable to cast to Job")
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
pub(crate) struct SpawnerInner {
    pub(crate) spawner: SpawnerBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct BeaverInner {
    pub(crate) beaver: BeaverBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct JobInner {
    pub(crate) job: JobBase,
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
    pub(crate) beavers: List<Beaver>,
    pub(crate) lodges: List<Tile>,
    pub(crate) branches_to_build_lodge: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct TileBase {
    pub(crate) x: i64,
    pub(crate) y: i64,
    pub(crate) tile_north: Option<Tile>,
    pub(crate) tile_east: Option<Tile>,
    pub(crate) tile_south: Option<Tile>,
    pub(crate) tile_west: Option<Tile>,
    pub(crate) type_: Str,
    pub(crate) flow_direction: Str,
    pub(crate) beaver: Option<Beaver>,
    pub(crate) spawner: Option<Spawner>,
    pub(crate) lodge_owner: Option<Player>,
    pub(crate) branches: i64,
    pub(crate) food: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct SpawnerBase {
    pub(crate) type_: Str,
    pub(crate) health: i64,
    pub(crate) tile: Tile,
    pub(crate) has_been_harvested: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct BeaverBase {
    pub(crate) moves: i64,
    pub(crate) owner: Player,
    pub(crate) actions: i64,
    pub(crate) tile: Option<Tile>,
    pub(crate) health: i64,
    pub(crate) turns_distracted: i64,
    pub(crate) branches: i64,
    pub(crate) food: i64,
    pub(crate) job: Job,
    pub(crate) recruited: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct JobBase {
    pub(crate) title: Str,
    pub(crate) health: i64,
    pub(crate) moves: i64,
    pub(crate) actions: i64,
    pub(crate) damage: i64,
    pub(crate) chopping: i64,
    pub(crate) munching: i64,
    pub(crate) distraction_power: i64,
    pub(crate) carry_limit: i64,
    pub(crate) cost: i64,
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
    pub(crate) beavers: List<Beaver>,
    pub(crate) spawner: List<Spawner>,
    pub(crate) spawner_types: List<Str>,
    pub(crate) jobs: List<Job>,
    pub(crate) free_beavers_count: i64,
    pub(crate) lodges_to_win: i64,
    pub(crate) lodge_cost_constant: f64,
    pub(crate) spawner_harvest_constant: f64,
}
