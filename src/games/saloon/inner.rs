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
    Tile(TileInner),
    Cowboy(CowboyInner),
    Furnishing(FurnishingInner),
    Bottle(BottleInner),
    YoungGun(YoungGunInner),
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
            GameObject::Tile(obj) => Some(&obj.game_object),
            GameObject::Cowboy(obj) => Some(&obj.game_object),
            GameObject::Furnishing(obj) => Some(&obj.game_object),
            GameObject::Bottle(obj) => Some(&obj.game_object),
            GameObject::YoungGun(obj) => Some(&obj.game_object),
        }
    }

    pub fn as_game_object(&self) -> &GameObjectBase {
        self.try_as_game_object().expect("unreachable: unable to cast to GameObject")
    }

    pub fn try_as_player(&self) -> Option< &PlayerBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(obj) => Some(&obj.player),
            GameObject::Tile(_obj) => None,
            GameObject::Cowboy(_obj) => None,
            GameObject::Furnishing(_obj) => None,
            GameObject::Bottle(_obj) => None,
            GameObject::YoungGun(_obj) => None,
        }
    }

    pub fn as_player(&self) -> &PlayerBase {
        self.try_as_player().expect("unreachable: unable to cast to Player")
    }

    pub fn try_as_tile(&self) -> Option< &TileBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(obj) => Some(&obj.tile),
            GameObject::Cowboy(_obj) => None,
            GameObject::Furnishing(_obj) => None,
            GameObject::Bottle(_obj) => None,
            GameObject::YoungGun(_obj) => None,
        }
    }

    pub fn as_tile(&self) -> &TileBase {
        self.try_as_tile().expect("unreachable: unable to cast to Tile")
    }

    pub fn try_as_cowboy(&self) -> Option< &CowboyBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Cowboy(obj) => Some(&obj.cowboy),
            GameObject::Furnishing(_obj) => None,
            GameObject::Bottle(_obj) => None,
            GameObject::YoungGun(_obj) => None,
        }
    }

    pub fn as_cowboy(&self) -> &CowboyBase {
        self.try_as_cowboy().expect("unreachable: unable to cast to Cowboy")
    }

    pub fn try_as_furnishing(&self) -> Option< &FurnishingBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Cowboy(_obj) => None,
            GameObject::Furnishing(obj) => Some(&obj.furnishing),
            GameObject::Bottle(_obj) => None,
            GameObject::YoungGun(_obj) => None,
        }
    }

    pub fn as_furnishing(&self) -> &FurnishingBase {
        self.try_as_furnishing().expect("unreachable: unable to cast to Furnishing")
    }

    pub fn try_as_bottle(&self) -> Option< &BottleBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Cowboy(_obj) => None,
            GameObject::Furnishing(_obj) => None,
            GameObject::Bottle(obj) => Some(&obj.bottle),
            GameObject::YoungGun(_obj) => None,
        }
    }

    pub fn as_bottle(&self) -> &BottleBase {
        self.try_as_bottle().expect("unreachable: unable to cast to Bottle")
    }

    pub fn try_as_young_gun(&self) -> Option< &YoungGunBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Tile(_obj) => None,
            GameObject::Cowboy(_obj) => None,
            GameObject::Furnishing(_obj) => None,
            GameObject::Bottle(_obj) => None,
            GameObject::YoungGun(obj) => Some(&obj.young_gun),
        }
    }

    pub fn as_young_gun(&self) -> &YoungGunBase {
        self.try_as_young_gun().expect("unreachable: unable to cast to YoungGun")
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
pub struct TileInner {
    pub tile: TileBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct CowboyInner {
    pub cowboy: CowboyBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct FurnishingInner {
    pub furnishing: FurnishingBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct BottleInner {
    pub bottle: BottleBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct YoungGunInner {
    pub young_gun: YoungGunBase,
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
    pub score: i64,
    pub rowdiness: i64,
    pub kills: i64,
    pub cowboys: List<Cowboy>,
    pub young_gun: YoungGun,
    pub siesta: i64,
}

#[derive(Debug, Clone)]
pub struct TileBase {
    pub x: i64,
    pub y: i64,
    pub tile_north: Option<Tile>,
    pub tile_east: Option<Tile>,
    pub tile_south: Option<Tile>,
    pub tile_west: Option<Tile>,
    pub cowboy: Option<Cowboy>,
    pub furnishing: Option<Furnishing>,
    pub is_balcony: bool,
    pub has_hazard: bool,
    pub bottle: Option<Bottle>,
    pub young_gun: Option<YoungGun>,
}

#[derive(Debug, Clone)]
pub struct CowboyBase {
    pub health: i64,
    pub owner: Player,
    pub is_dead: bool,
    pub job: Str,
    pub can_move: bool,
    pub tile: Option<Tile>,
    pub focus: i64,
    pub is_drunk: bool,
    pub drunk_direction: Str,
    pub tolerance: i64,
    pub turns_busy: i64,
}

#[derive(Debug, Clone)]
pub struct FurnishingBase {
    pub is_piano: bool,
    pub tile: Option<Tile>,
    pub health: i64,
    pub is_destroyed: bool,
    pub is_playing: bool,
}

#[derive(Debug, Clone)]
pub struct BottleBase {
    pub tile: Option<Tile>,
    pub direction: Str,
    pub is_destroyed: bool,
    pub drunk_direction: Str,
}

#[derive(Debug, Clone)]
pub struct YoungGunBase {
    pub owner: Player,
    pub tile: Tile,
    pub can_call_in: bool,
    pub call_in_tile: Tile,
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
    pub map_width: i64,
    pub map_height: i64,
    pub tiles: List<Tile>,
    pub cowboys: List<Cowboy>,
    pub furnishings: List<Furnishing>,
    pub jobs: List<Str>,
    pub bottles: List<Bottle>,
    pub rowdiness_to_siesta: i64,
    pub siesta_length: i64,
    pub max_cowboys_per_job: i64,
    pub sharpshooter_damage: i64,
    pub brawler_damage: i64,
    pub turns_drunk: i64,
    pub bartender_cooldown: i64,
}
