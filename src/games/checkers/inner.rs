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
    Checker(CheckerInner),
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
            AnyGameObject::Checker(obj) => Some(&obj.game_object),
        }
    }

    pub fn as_game_object(&self) -> &GameObjectBase {
        self.try_as_game_object().expect("unreachable: unable to cast to GameObject")
    }

    pub fn try_as_player(&self) -> Option< &PlayerBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(obj) => Some(&obj.player),
            AnyGameObject::Checker(_obj) => None,
        }
    }

    pub fn as_player(&self) -> &PlayerBase {
        self.try_as_player().expect("unreachable: unable to cast to Player")
    }

    pub fn try_as_checker(&self) -> Option< &CheckerBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Checker(obj) => Some(&obj.checker),
        }
    }

    pub fn as_checker(&self) -> &CheckerBase {
        self.try_as_checker().expect("unreachable: unable to cast to Checker")
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
pub struct CheckerInner {
    pub checker: CheckerBase,
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
    pub checkers: List<Checker>,
    pub y_direction: i64,
}

#[derive(Debug, Clone)]
pub struct CheckerBase {
    pub owner: Player,
    pub x: i64,
    pub y: i64,
    pub kinged: bool,
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
    pub board_width: i64,
    pub board_height: i64,
    pub checkers: List<Checker>,
    pub checker_moved: Option<Checker>,
    pub checker_moved_jumped: bool,
}
