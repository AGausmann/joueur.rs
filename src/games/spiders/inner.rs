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

    pub(crate) fn as_game_object(&self) -> &GameObjectBase {
        self.try_as_game_object().expect("unreachable: unable to cast to GameObject")
    }

    pub(crate) fn try_as_player(&self) -> Option< &PlayerBase > {
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

    pub(crate) fn as_player(&self) -> &PlayerBase {
        self.try_as_player().expect("unreachable: unable to cast to Player")
    }

    pub(crate) fn try_as_nest(&self) -> Option< &NestBase > {
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

    pub(crate) fn as_nest(&self) -> &NestBase {
        self.try_as_nest().expect("unreachable: unable to cast to Nest")
    }

    pub(crate) fn try_as_web(&self) -> Option< &WebBase > {
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

    pub(crate) fn as_web(&self) -> &WebBase {
        self.try_as_web().expect("unreachable: unable to cast to Web")
    }

    pub(crate) fn try_as_spider(&self) -> Option< &SpiderBase > {
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

    pub(crate) fn as_spider(&self) -> &SpiderBase {
        self.try_as_spider().expect("unreachable: unable to cast to Spider")
    }

    pub(crate) fn try_as_brood_mother(&self) -> Option< &BroodMotherBase > {
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

    pub(crate) fn as_brood_mother(&self) -> &BroodMotherBase {
        self.try_as_brood_mother().expect("unreachable: unable to cast to BroodMother")
    }

    pub(crate) fn try_as_spiderling(&self) -> Option< &SpiderlingBase > {
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

    pub(crate) fn as_spiderling(&self) -> &SpiderlingBase {
        self.try_as_spiderling().expect("unreachable: unable to cast to Spiderling")
    }

    pub(crate) fn try_as_spitter(&self) -> Option< &SpitterBase > {
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

    pub(crate) fn as_spitter(&self) -> &SpitterBase {
        self.try_as_spitter().expect("unreachable: unable to cast to Spitter")
    }

    pub(crate) fn try_as_weaver(&self) -> Option< &WeaverBase > {
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

    pub(crate) fn as_weaver(&self) -> &WeaverBase {
        self.try_as_weaver().expect("unreachable: unable to cast to Weaver")
    }

    pub(crate) fn try_as_cutter(&self) -> Option< &CutterBase > {
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

    pub(crate) fn as_cutter(&self) -> &CutterBase {
        self.try_as_cutter().expect("unreachable: unable to cast to Cutter")
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
pub(crate) struct NestInner {
    pub(crate) nest: NestBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct WebInner {
    pub(crate) web: WebBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct SpiderInner {
    pub(crate) spider: SpiderBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct BroodMotherInner {
    pub(crate) brood_mother: BroodMotherBase,
    pub(crate) spider: SpiderBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct SpiderlingInner {
    pub(crate) spiderling: SpiderlingBase,
    pub(crate) spider: SpiderBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct SpitterInner {
    pub(crate) spitter: SpitterBase,
    pub(crate) spiderling: SpiderlingBase,
    pub(crate) spider: SpiderBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct WeaverInner {
    pub(crate) weaver: WeaverBase,
    pub(crate) spiderling: SpiderlingBase,
    pub(crate) spider: SpiderBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct CutterInner {
    pub(crate) cutter: CutterBase,
    pub(crate) spiderling: SpiderlingBase,
    pub(crate) spider: SpiderBase,
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
    pub(crate) spiders: List<Spider>,
    pub(crate) brood_mother: BroodMother,
    pub(crate) max_spiderlings: i64,
    pub(crate) number_of_nests_controlled: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct NestBase {
    pub(crate) webs: List<Web>,
    pub(crate) spiders: List<Spider>,
    pub(crate) controlling_player: Option<Player>,
    pub(crate) x: i64,
    pub(crate) y: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct WebBase {
    pub(crate) nest_a: Option<Nest>,
    pub(crate) nest_b: Option<Nest>,
    pub(crate) spiderlings: List<Spiderling>,
    pub(crate) strength: i64,
    pub(crate) load: i64,
    pub(crate) length: f64,
}

#[derive(Debug, Clone)]
pub(crate) struct SpiderBase {
    pub(crate) owner: Player,
    pub(crate) nest: Option<Nest>,
    pub(crate) is_dead: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct BroodMotherBase {
    pub(crate) health: i64,
    pub(crate) eggs: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct SpiderlingBase {
    pub(crate) busy: Str,
    pub(crate) work_remaining: f64,
    pub(crate) number_of_coworkers: i64,
    pub(crate) moving_on_web: Option<Web>,
    pub(crate) moving_to_nest: Option<Nest>,
}

#[derive(Debug, Clone)]
pub(crate) struct SpitterBase {
    pub(crate) spitting_web_to_nest: Option<Nest>,
}

#[derive(Debug, Clone)]
pub(crate) struct WeaverBase {
    pub(crate) strengthening_web: Option<Web>,
    pub(crate) weakening_web: Option<Web>,
}

#[derive(Debug, Clone)]
pub(crate) struct CutterBase {
    pub(crate) cutting_web: Option<Web>,
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
    pub(crate) nests: List<Nest>,
    pub(crate) webs: List<Web>,
    pub(crate) movement_speed: i64,
    pub(crate) weave_speed: i64,
    pub(crate) cut_speed: i64,
    pub(crate) spit_speed: i64,
    pub(crate) weave_power: i64,
    pub(crate) initial_web_strength: i64,
    pub(crate) max_web_strength: i64,
    pub(crate) eggs_scalar: f64,
}
