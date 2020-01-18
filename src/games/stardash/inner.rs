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
    Body(BodyInner),
    Projectile(ProjectileInner),
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
            GameObject::Body(obj) => Some(&obj.game_object),
            GameObject::Projectile(obj) => Some(&obj.game_object),
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
            GameObject::Body(_obj) => None,
            GameObject::Projectile(_obj) => None,
            GameObject::Unit(_obj) => None,
            GameObject::Job(_obj) => None,
        }
    }

    pub(crate) fn as_player(&self) -> &PlayerBase {
        self.try_as_player().expect("unreachable: unable to cast to Player")
    }

    pub(crate) fn try_as_body(&self) -> Option< &BodyBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Body(obj) => Some(&obj.body),
            GameObject::Projectile(_obj) => None,
            GameObject::Unit(_obj) => None,
            GameObject::Job(_obj) => None,
        }
    }

    pub(crate) fn as_body(&self) -> &BodyBase {
        self.try_as_body().expect("unreachable: unable to cast to Body")
    }

    pub(crate) fn try_as_projectile(&self) -> Option< &ProjectileBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Body(_obj) => None,
            GameObject::Projectile(obj) => Some(&obj.projectile),
            GameObject::Unit(_obj) => None,
            GameObject::Job(_obj) => None,
        }
    }

    pub(crate) fn as_projectile(&self) -> &ProjectileBase {
        self.try_as_projectile().expect("unreachable: unable to cast to Projectile")
    }

    pub(crate) fn try_as_unit(&self) -> Option< &UnitBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Body(_obj) => None,
            GameObject::Projectile(_obj) => None,
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
            GameObject::Body(_obj) => None,
            GameObject::Projectile(_obj) => None,
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
pub(crate) struct BodyInner {
    pub(crate) body: BodyBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct ProjectileInner {
    pub(crate) projectile: ProjectileBase,
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
    pub(crate) projectiles: List<Projectile>,
    pub(crate) money: i64,
    pub(crate) home_base: Body,
    pub(crate) victory_points: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct BodyBase {
    pub(crate) owner: Option<Player>,
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) radius: f64,
    pub(crate) body_type: Str,
    pub(crate) material_type: Str,
    pub(crate) amount: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct ProjectileBase {
    pub(crate) owner: Option<Player>,
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) target: Unit,
    pub(crate) fuel: i64,
    pub(crate) energy: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct UnitBase {
    pub(crate) owner: Option<Player>,
    pub(crate) x: f64,
    pub(crate) y: f64,
    pub(crate) dash_x: f64,
    pub(crate) dash_y: f64,
    pub(crate) job: Job,
    pub(crate) energy: i64,
    pub(crate) shield: i64,
    pub(crate) acted: bool,
    pub(crate) moves: f64,
    pub(crate) genarium: i64,
    pub(crate) rarium: i64,
    pub(crate) legendarium: i64,
    pub(crate) mythicite: i64,
    pub(crate) is_busy: bool,
    pub(crate) protector: Option<Unit>,
}

#[derive(Debug, Clone)]
pub(crate) struct JobBase {
    pub(crate) title: Str,
    pub(crate) energy: i64,
    pub(crate) shield: i64,
    pub(crate) moves: i64,
    pub(crate) damage: i64,
    pub(crate) carry_limit: i64,
    pub(crate) unit_cost: i64,
    pub(crate) range: i64,
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
    pub(crate) units: List<Unit>,
    pub(crate) projectiles: List<Projectile>,
    pub(crate) jobs: List<Job>,
    pub(crate) bodies: List<Body>,
    pub(crate) size_x: i64,
    pub(crate) size_y: i64,
    pub(crate) dash_distance: i64,
    pub(crate) dash_cost: i64,
    pub(crate) max_asteroid: i64,
    pub(crate) min_asteroid: i64,
    pub(crate) ore_rarity_genarium: f64,
    pub(crate) ore_rarity_rarium: f64,
    pub(crate) ore_rarity_legendarium: f64,
    pub(crate) genarium_value: f64,
    pub(crate) rarium_value: f64,
    pub(crate) legendarium_value: f64,
    pub(crate) mythicite_amount: f64,
    pub(crate) regenerate_rate: f64,
    pub(crate) planet_recharge_rate: i64,
    pub(crate) planet_energy_cap: i64,
    pub(crate) mining_speed: i64,
    pub(crate) projectile_speed: i64,
    pub(crate) projectile_radius: i64,
    pub(crate) ship_radius: i64,
    pub(crate) turns_to_orbit: i64,
    pub(crate) orbits_protected: i64,
}
