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
        }
    }

    pub(crate) fn as_game_object(&self) -> &GameObjectBase {
        self.try_as_game_object().expect("unreachable: unable to cast to GameObject")
    }

    pub(crate) fn try_as_player(&self) -> Option< &PlayerBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(obj) => Some(&obj.player),
        }
    }

    pub(crate) fn as_player(&self) -> &PlayerBase {
        self.try_as_player().expect("unreachable: unable to cast to Player")
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
    pub(crate) color: Str,
}

#[derive(Debug, Clone)]
pub(crate) struct GameBase {
    pub(crate) game_objects: Map<Str, GameObject>,
    pub(crate) players: List<Player>,
    pub(crate) session: Str,
    pub(crate) history: List<Str>,
    pub(crate) fen: Str,
}
