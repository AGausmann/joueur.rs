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
    Nest(NestInner),
    Web(WebInner),
    Spider(SpiderInner),
    BroodMother(BroodMotherInner),
    Spiderling(SpiderlingInner),
    Spitter(SpitterInner),
    Weaver(WeaverInner),
    Cutter(CutterInner),
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
            AnyGameObject::Nest(obj) => Some(&obj.game_object),
            AnyGameObject::Web(obj) => Some(&obj.game_object),
            AnyGameObject::Spider(obj) => Some(&obj.game_object),
            AnyGameObject::BroodMother(obj) => Some(&obj.game_object),
            AnyGameObject::Spiderling(obj) => Some(&obj.game_object),
            AnyGameObject::Spitter(obj) => Some(&obj.game_object),
            AnyGameObject::Weaver(obj) => Some(&obj.game_object),
            AnyGameObject::Cutter(obj) => Some(&obj.game_object),
        }
    }

    pub fn as_game_object(&self) -> &GameObjectBase {
        self.try_as_game_object().expect("unreachable: unable to cast to GameObject")
    }

    pub fn try_as_player(&self) -> Option< &PlayerBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(obj) => Some(&obj.player),
            AnyGameObject::Nest(_obj) => None,
            AnyGameObject::Web(_obj) => None,
            AnyGameObject::Spider(_obj) => None,
            AnyGameObject::BroodMother(_obj) => None,
            AnyGameObject::Spiderling(_obj) => None,
            AnyGameObject::Spitter(_obj) => None,
            AnyGameObject::Weaver(_obj) => None,
            AnyGameObject::Cutter(_obj) => None,
        }
    }

    pub fn as_player(&self) -> &PlayerBase {
        self.try_as_player().expect("unreachable: unable to cast to Player")
    }

    pub fn try_as_nest(&self) -> Option< &NestBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Nest(obj) => Some(&obj.nest),
            AnyGameObject::Web(_obj) => None,
            AnyGameObject::Spider(_obj) => None,
            AnyGameObject::BroodMother(_obj) => None,
            AnyGameObject::Spiderling(_obj) => None,
            AnyGameObject::Spitter(_obj) => None,
            AnyGameObject::Weaver(_obj) => None,
            AnyGameObject::Cutter(_obj) => None,
        }
    }

    pub fn as_nest(&self) -> &NestBase {
        self.try_as_nest().expect("unreachable: unable to cast to Nest")
    }

    pub fn try_as_web(&self) -> Option< &WebBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Nest(_obj) => None,
            AnyGameObject::Web(obj) => Some(&obj.web),
            AnyGameObject::Spider(_obj) => None,
            AnyGameObject::BroodMother(_obj) => None,
            AnyGameObject::Spiderling(_obj) => None,
            AnyGameObject::Spitter(_obj) => None,
            AnyGameObject::Weaver(_obj) => None,
            AnyGameObject::Cutter(_obj) => None,
        }
    }

    pub fn as_web(&self) -> &WebBase {
        self.try_as_web().expect("unreachable: unable to cast to Web")
    }

    pub fn try_as_spider(&self) -> Option< &SpiderBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Nest(_obj) => None,
            AnyGameObject::Web(_obj) => None,
            AnyGameObject::Spider(obj) => Some(&obj.spider),
            AnyGameObject::BroodMother(obj) => Some(&obj.spider),
            AnyGameObject::Spiderling(obj) => Some(&obj.spider),
            AnyGameObject::Spitter(obj) => Some(&obj.spider),
            AnyGameObject::Weaver(obj) => Some(&obj.spider),
            AnyGameObject::Cutter(obj) => Some(&obj.spider),
        }
    }

    pub fn as_spider(&self) -> &SpiderBase {
        self.try_as_spider().expect("unreachable: unable to cast to Spider")
    }

    pub fn try_as_brood_mother(&self) -> Option< &BroodMotherBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Nest(_obj) => None,
            AnyGameObject::Web(_obj) => None,
            AnyGameObject::Spider(_obj) => None,
            AnyGameObject::BroodMother(obj) => Some(&obj.brood_mother),
            AnyGameObject::Spiderling(_obj) => None,
            AnyGameObject::Spitter(_obj) => None,
            AnyGameObject::Weaver(_obj) => None,
            AnyGameObject::Cutter(_obj) => None,
        }
    }

    pub fn as_brood_mother(&self) -> &BroodMotherBase {
        self.try_as_brood_mother().expect("unreachable: unable to cast to BroodMother")
    }

    pub fn try_as_spiderling(&self) -> Option< &SpiderlingBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Nest(_obj) => None,
            AnyGameObject::Web(_obj) => None,
            AnyGameObject::Spider(_obj) => None,
            AnyGameObject::BroodMother(_obj) => None,
            AnyGameObject::Spiderling(obj) => Some(&obj.spiderling),
            AnyGameObject::Spitter(obj) => Some(&obj.spiderling),
            AnyGameObject::Weaver(obj) => Some(&obj.spiderling),
            AnyGameObject::Cutter(obj) => Some(&obj.spiderling),
        }
    }

    pub fn as_spiderling(&self) -> &SpiderlingBase {
        self.try_as_spiderling().expect("unreachable: unable to cast to Spiderling")
    }

    pub fn try_as_spitter(&self) -> Option< &SpitterBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Nest(_obj) => None,
            AnyGameObject::Web(_obj) => None,
            AnyGameObject::Spider(_obj) => None,
            AnyGameObject::BroodMother(_obj) => None,
            AnyGameObject::Spiderling(_obj) => None,
            AnyGameObject::Spitter(obj) => Some(&obj.spitter),
            AnyGameObject::Weaver(_obj) => None,
            AnyGameObject::Cutter(_obj) => None,
        }
    }

    pub fn as_spitter(&self) -> &SpitterBase {
        self.try_as_spitter().expect("unreachable: unable to cast to Spitter")
    }

    pub fn try_as_weaver(&self) -> Option< &WeaverBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Nest(_obj) => None,
            AnyGameObject::Web(_obj) => None,
            AnyGameObject::Spider(_obj) => None,
            AnyGameObject::BroodMother(_obj) => None,
            AnyGameObject::Spiderling(_obj) => None,
            AnyGameObject::Spitter(_obj) => None,
            AnyGameObject::Weaver(obj) => Some(&obj.weaver),
            AnyGameObject::Cutter(_obj) => None,
        }
    }

    pub fn as_weaver(&self) -> &WeaverBase {
        self.try_as_weaver().expect("unreachable: unable to cast to Weaver")
    }

    pub fn try_as_cutter(&self) -> Option< &CutterBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Nest(_obj) => None,
            AnyGameObject::Web(_obj) => None,
            AnyGameObject::Spider(_obj) => None,
            AnyGameObject::BroodMother(_obj) => None,
            AnyGameObject::Spiderling(_obj) => None,
            AnyGameObject::Spitter(_obj) => None,
            AnyGameObject::Weaver(_obj) => None,
            AnyGameObject::Cutter(obj) => Some(&obj.cutter),
        }
    }

    pub fn as_cutter(&self) -> &CutterBase {
        self.try_as_cutter().expect("unreachable: unable to cast to Cutter")
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
pub struct NestInner {
    pub nest: NestBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct WebInner {
    pub web: WebBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct SpiderInner {
    pub spider: SpiderBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct BroodMotherInner {
    pub brood_mother: BroodMotherBase,
    pub spider: SpiderBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct SpiderlingInner {
    pub spiderling: SpiderlingBase,
    pub spider: SpiderBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct SpitterInner {
    pub spitter: SpitterBase,
    pub spiderling: SpiderlingBase,
    pub spider: SpiderBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct WeaverInner {
    pub weaver: WeaverBase,
    pub spiderling: SpiderlingBase,
    pub spider: SpiderBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct CutterInner {
    pub cutter: CutterBase,
    pub spiderling: SpiderlingBase,
    pub spider: SpiderBase,
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
    pub spiders: List<Spider>,
    pub brood_mother: BroodMother,
    pub max_spiderlings: i64,
    pub number_of_nests_controlled: i64,
}

#[derive(Debug, Clone)]
pub struct NestBase {
    pub webs: List<Web>,
    pub spiders: List<Spider>,
    pub controlling_player: Option<Player>,
    pub x: i64,
    pub y: i64,
}

#[derive(Debug, Clone)]
pub struct WebBase {
    pub nest_a: Option<Nest>,
    pub nest_b: Option<Nest>,
    pub spiderlings: List<Spiderling>,
    pub strength: i64,
    pub load: i64,
    pub length: f64,
}

#[derive(Debug, Clone)]
pub struct SpiderBase {
    pub owner: Player,
    pub nest: Option<Nest>,
    pub is_dead: bool,
}

#[derive(Debug, Clone)]
pub struct BroodMotherBase {
    pub health: i64,
    pub eggs: i64,
}

#[derive(Debug, Clone)]
pub struct SpiderlingBase {
    pub busy: Str,
    pub work_remaining: f64,
    pub number_of_coworkers: i64,
    pub moving_on_web: Option<Web>,
    pub moving_to_nest: Option<Nest>,
}

#[derive(Debug, Clone)]
pub struct SpitterBase {
    pub spitting_web_to_nest: Option<Nest>,
}

#[derive(Debug, Clone)]
pub struct WeaverBase {
    pub strengthening_web: Option<Web>,
    pub weakening_web: Option<Web>,
}

#[derive(Debug, Clone)]
pub struct CutterBase {
    pub cutting_web: Option<Web>,
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
    pub nests: List<Nest>,
    pub webs: List<Web>,
    pub movement_speed: i64,
    pub weave_speed: i64,
    pub cut_speed: i64,
    pub spit_speed: i64,
    pub weave_power: i64,
    pub initial_web_strength: i64,
    pub max_web_strength: i64,
    pub eggs_scalar: f64,
}
