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
    Body(BodyInner),
    Projectile(ProjectileInner),
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
            AnyGameObject::Body(obj) => Some(&obj.game_object),
            AnyGameObject::Projectile(obj) => Some(&obj.game_object),
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
            AnyGameObject::Body(_obj) => None,
            AnyGameObject::Projectile(_obj) => None,
            AnyGameObject::Unit(_obj) => None,
            AnyGameObject::Job(_obj) => None,
        }
    }

    pub fn as_player(&self) -> &PlayerBase {
        self.try_as_player().expect("unreachable: unable to cast to Player")
    }

    pub fn try_as_body(&self) -> Option< &BodyBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Body(obj) => Some(&obj.body),
            AnyGameObject::Projectile(_obj) => None,
            AnyGameObject::Unit(_obj) => None,
            AnyGameObject::Job(_obj) => None,
        }
    }

    pub fn as_body(&self) -> &BodyBase {
        self.try_as_body().expect("unreachable: unable to cast to Body")
    }

    pub fn try_as_projectile(&self) -> Option< &ProjectileBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Body(_obj) => None,
            AnyGameObject::Projectile(obj) => Some(&obj.projectile),
            AnyGameObject::Unit(_obj) => None,
            AnyGameObject::Job(_obj) => None,
        }
    }

    pub fn as_projectile(&self) -> &ProjectileBase {
        self.try_as_projectile().expect("unreachable: unable to cast to Projectile")
    }

    pub fn try_as_unit(&self) -> Option< &UnitBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Body(_obj) => None,
            AnyGameObject::Projectile(_obj) => None,
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
            AnyGameObject::Body(_obj) => None,
            AnyGameObject::Projectile(_obj) => None,
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
pub struct BodyInner {
    pub body: BodyBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct ProjectileInner {
    pub projectile: ProjectileBase,
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
    pub projectiles: List<Projectile>,
    pub money: i64,
    pub home_base: Body,
    pub victory_points: i64,
}

#[derive(Debug, Clone)]
pub struct BodyBase {
    pub owner: Option<Player>,
    pub x: f64,
    pub y: f64,
    pub radius: f64,
    pub body_type: Str,
    pub material_type: Str,
    pub amount: i64,
}

#[derive(Debug, Clone)]
pub struct ProjectileBase {
    pub owner: Option<Player>,
    pub x: f64,
    pub y: f64,
    pub target: Unit,
    pub fuel: i64,
    pub energy: i64,
}

#[derive(Debug, Clone)]
pub struct UnitBase {
    pub owner: Option<Player>,
    pub x: f64,
    pub y: f64,
    pub dash_x: f64,
    pub dash_y: f64,
    pub job: Job,
    pub energy: i64,
    pub shield: i64,
    pub acted: bool,
    pub moves: f64,
    pub genarium: i64,
    pub rarium: i64,
    pub legendarium: i64,
    pub mythicite: i64,
    pub is_busy: bool,
    pub protector: Option<Unit>,
}

#[derive(Debug, Clone)]
pub struct JobBase {
    pub title: Str,
    pub energy: i64,
    pub shield: i64,
    pub moves: i64,
    pub damage: i64,
    pub carry_limit: i64,
    pub unit_cost: i64,
    pub range: i64,
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
    pub units: List<Unit>,
    pub projectiles: List<Projectile>,
    pub jobs: List<Job>,
    pub bodies: List<Body>,
    pub size_x: i64,
    pub size_y: i64,
    pub dash_distance: i64,
    pub dash_cost: i64,
    pub max_asteroid: i64,
    pub min_asteroid: i64,
    pub ore_rarity_genarium: f64,
    pub ore_rarity_rarium: f64,
    pub ore_rarity_legendarium: f64,
    pub genarium_value: f64,
    pub rarium_value: f64,
    pub legendarium_value: f64,
    pub mythicite_amount: f64,
    pub regenerate_rate: f64,
    pub planet_recharge_rate: i64,
    pub planet_energy_cap: i64,
    pub mining_speed: i64,
    pub projectile_speed: i64,
    pub projectile_radius: i64,
    pub ship_radius: i64,
    pub turns_to_orbit: i64,
    pub orbits_protected: i64,
}
