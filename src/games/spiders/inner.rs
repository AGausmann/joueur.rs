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
    Nest(NestInner),
    Web(WebInner),
    Spider(SpiderInner),
    BroodMother(BroodMotherInner),
    Spiderling(SpiderlingInner),
    Spitter(SpitterInner),
    Weaver(WeaverInner),
    Cutter(CutterInner),
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
            GameObject::Nest(obj) => Some(&obj.game_object),
            GameObject::Web(obj) => Some(&obj.game_object),
            GameObject::Spider(obj) => Some(&obj.game_object),
            GameObject::BroodMother(obj) => Some(&obj.game_object),
            GameObject::Spiderling(obj) => Some(&obj.game_object),
            GameObject::Spitter(obj) => Some(&obj.game_object),
            GameObject::Weaver(obj) => Some(&obj.game_object),
            GameObject::Cutter(obj) => Some(&obj.game_object),
        }
    }

    pub fn as_game_object(&self) -> &GameObjectBase {
        self.try_as_game_object().expect("unreachable: unable to cast to GameObject")
    }

    pub fn try_as_player(&self) -> Option< &PlayerBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(obj) => Some(&obj.player),
            GameObject::Nest(_obj) => None,
            GameObject::Web(_obj) => None,
            GameObject::Spider(_obj) => None,
            GameObject::BroodMother(_obj) => None,
            GameObject::Spiderling(_obj) => None,
            GameObject::Spitter(_obj) => None,
            GameObject::Weaver(_obj) => None,
            GameObject::Cutter(_obj) => None,
        }
    }

    pub fn as_player(&self) -> &PlayerBase {
        self.try_as_player().expect("unreachable: unable to cast to Player")
    }

    pub fn try_as_nest(&self) -> Option< &NestBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Nest(obj) => Some(&obj.nest),
            GameObject::Web(_obj) => None,
            GameObject::Spider(_obj) => None,
            GameObject::BroodMother(_obj) => None,
            GameObject::Spiderling(_obj) => None,
            GameObject::Spitter(_obj) => None,
            GameObject::Weaver(_obj) => None,
            GameObject::Cutter(_obj) => None,
        }
    }

    pub fn as_nest(&self) -> &NestBase {
        self.try_as_nest().expect("unreachable: unable to cast to Nest")
    }

    pub fn try_as_web(&self) -> Option< &WebBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Nest(_obj) => None,
            GameObject::Web(obj) => Some(&obj.web),
            GameObject::Spider(_obj) => None,
            GameObject::BroodMother(_obj) => None,
            GameObject::Spiderling(_obj) => None,
            GameObject::Spitter(_obj) => None,
            GameObject::Weaver(_obj) => None,
            GameObject::Cutter(_obj) => None,
        }
    }

    pub fn as_web(&self) -> &WebBase {
        self.try_as_web().expect("unreachable: unable to cast to Web")
    }

    pub fn try_as_spider(&self) -> Option< &SpiderBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Nest(_obj) => None,
            GameObject::Web(_obj) => None,
            GameObject::Spider(obj) => Some(&obj.spider),
            GameObject::BroodMother(obj) => Some(&obj.spider),
            GameObject::Spiderling(obj) => Some(&obj.spider),
            GameObject::Spitter(obj) => Some(&obj.spider),
            GameObject::Weaver(obj) => Some(&obj.spider),
            GameObject::Cutter(obj) => Some(&obj.spider),
        }
    }

    pub fn as_spider(&self) -> &SpiderBase {
        self.try_as_spider().expect("unreachable: unable to cast to Spider")
    }

    pub fn try_as_brood_mother(&self) -> Option< &BroodMotherBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Nest(_obj) => None,
            GameObject::Web(_obj) => None,
            GameObject::Spider(_obj) => None,
            GameObject::BroodMother(obj) => Some(&obj.brood_mother),
            GameObject::Spiderling(_obj) => None,
            GameObject::Spitter(_obj) => None,
            GameObject::Weaver(_obj) => None,
            GameObject::Cutter(_obj) => None,
        }
    }

    pub fn as_brood_mother(&self) -> &BroodMotherBase {
        self.try_as_brood_mother().expect("unreachable: unable to cast to BroodMother")
    }

    pub fn try_as_spiderling(&self) -> Option< &SpiderlingBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Nest(_obj) => None,
            GameObject::Web(_obj) => None,
            GameObject::Spider(_obj) => None,
            GameObject::BroodMother(_obj) => None,
            GameObject::Spiderling(obj) => Some(&obj.spiderling),
            GameObject::Spitter(obj) => Some(&obj.spiderling),
            GameObject::Weaver(obj) => Some(&obj.spiderling),
            GameObject::Cutter(obj) => Some(&obj.spiderling),
        }
    }

    pub fn as_spiderling(&self) -> &SpiderlingBase {
        self.try_as_spiderling().expect("unreachable: unable to cast to Spiderling")
    }

    pub fn try_as_spitter(&self) -> Option< &SpitterBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Nest(_obj) => None,
            GameObject::Web(_obj) => None,
            GameObject::Spider(_obj) => None,
            GameObject::BroodMother(_obj) => None,
            GameObject::Spiderling(_obj) => None,
            GameObject::Spitter(obj) => Some(&obj.spitter),
            GameObject::Weaver(_obj) => None,
            GameObject::Cutter(_obj) => None,
        }
    }

    pub fn as_spitter(&self) -> &SpitterBase {
        self.try_as_spitter().expect("unreachable: unable to cast to Spitter")
    }

    pub fn try_as_weaver(&self) -> Option< &WeaverBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Nest(_obj) => None,
            GameObject::Web(_obj) => None,
            GameObject::Spider(_obj) => None,
            GameObject::BroodMother(_obj) => None,
            GameObject::Spiderling(_obj) => None,
            GameObject::Spitter(_obj) => None,
            GameObject::Weaver(obj) => Some(&obj.weaver),
            GameObject::Cutter(_obj) => None,
        }
    }

    pub fn as_weaver(&self) -> &WeaverBase {
        self.try_as_weaver().expect("unreachable: unable to cast to Weaver")
    }

    pub fn try_as_cutter(&self) -> Option< &CutterBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Nest(_obj) => None,
            GameObject::Web(_obj) => None,
            GameObject::Spider(_obj) => None,
            GameObject::BroodMother(_obj) => None,
            GameObject::Spiderling(_obj) => None,
            GameObject::Spitter(_obj) => None,
            GameObject::Weaver(_obj) => None,
            GameObject::Cutter(obj) => Some(&obj.cutter),
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
