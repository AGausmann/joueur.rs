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
    Machine(MachineInner),
    Unit(UnitInner),
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
            GameObject::Machine(obj) => Some(&obj.game_object),
            GameObject::Unit(obj) => Some(&obj.game_object),
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
            GameObject::Machine(_obj) => None,
            GameObject::Unit(_obj) => None,
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
            GameObject::Machine(_obj) => None,
            GameObject::Unit(_obj) => None,
            GameObject::Job(_obj) => None,
        }
    }

    pub(crate) fn as_tile(&self) -> &TileBase {
        self.try_as_tile().expect("unreachable: unable to cast to Tile")
    }

    pub(crate) fn try_as_machine(&self) -> Option< &MachineBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Machine(obj) => Some(&obj.machine),
            GameObject::Unit(_obj) => None,
            GameObject::Job(_obj) => None,
        }
    }

    pub(crate) fn as_machine(&self) -> &MachineBase {
        self.try_as_machine().expect("unreachable: unable to cast to Machine")
    }

    pub(crate) fn try_as_unit(&self) -> Option< &UnitBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Machine(_obj) => None,
            GameObject::Unit(obj) => Some(&obj.unit),
            GameObject::Job(_obj) => None,
        }
    }

    pub(crate) fn as_unit(&self) -> &UnitBase {
        self.try_as_unit().expect("unreachable: unable to cast to Unit")
    }

    pub(crate) fn try_as_job(&self) -> Option< &JobBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Machine(_obj) => None,
            GameObject::Unit(_obj) => None,
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
pub(crate) struct MachineInner {
    pub(crate) machine: MachineBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct UnitInner {
    pub(crate) unit: UnitBase,
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
    pub(crate) units: List<Unit>,
    pub(crate) heat: i64,
    pub(crate) pressure: i64,
    pub(crate) intern_spawn: i64,
    pub(crate) physicist_spawn: i64,
    pub(crate) manager_spawn: i64,
    pub(crate) spawn_tiles: List<Tile>,
    pub(crate) generator_tiles: List<Tile>,
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
    pub(crate) machine: Option<Machine>,
    pub(crate) is_wall: bool,
    pub(crate) type_: Str,
    pub(crate) direction: Str,
    pub(crate) owner: Option<Player>,
    pub(crate) redium_ore: i64,
    pub(crate) redium: i64,
    pub(crate) blueium_ore: i64,
    pub(crate) blueium: i64,
    pub(crate) decoration: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct MachineBase {
    pub(crate) tile: Tile,
    pub(crate) worked: i64,
    pub(crate) ore_type: Str,
    pub(crate) refine_time: i64,
    pub(crate) refine_input: i64,
    pub(crate) refine_output: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct UnitBase {
    pub(crate) owner: Option<Player>,
    pub(crate) tile: Option<Tile>,
    pub(crate) job: Job,
    pub(crate) health: i64,
    pub(crate) acted: bool,
    pub(crate) moves: i64,
    pub(crate) redium_ore: i64,
    pub(crate) redium: i64,
    pub(crate) blueium_ore: i64,
    pub(crate) blueium: i64,
    pub(crate) stun_time: i64,
    pub(crate) stun_immune: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct JobBase {
    pub(crate) title: Str,
    pub(crate) health: i64,
    pub(crate) moves: i64,
    pub(crate) damage: i64,
    pub(crate) carry_limit: i64,
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
    pub(crate) jobs: List<Job>,
    pub(crate) machines: List<Machine>,
    pub(crate) refined_value: i64,
    pub(crate) spawn_time: i64,
    pub(crate) manager_cap: i64,
    pub(crate) intern_cap: i64,
    pub(crate) physicist_cap: i64,
    pub(crate) stun_time: i64,
    pub(crate) time_immune: i64,
    pub(crate) material_spawn: i64,
    pub(crate) regenerate_rate: f64,
    pub(crate) victory_amount: i64,
}
