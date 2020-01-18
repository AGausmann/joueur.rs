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
    Cowboy(CowboyInner),
    Furnishing(FurnishingInner),
    Bottle(BottleInner),
    YoungGun(YoungGunInner),
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
            GameObject::Cowboy(obj) => Some(&obj.game_object),
            GameObject::Furnishing(obj) => Some(&obj.game_object),
            GameObject::Bottle(obj) => Some(&obj.game_object),
            GameObject::YoungGun(obj) => Some(&obj.game_object),
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
            GameObject::Cowboy(_obj) => None,
            GameObject::Furnishing(_obj) => None,
            GameObject::Bottle(_obj) => None,
            GameObject::YoungGun(_obj) => None,
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
            GameObject::Cowboy(_obj) => None,
            GameObject::Furnishing(_obj) => None,
            GameObject::Bottle(_obj) => None,
            GameObject::YoungGun(_obj) => None,
        }
    }

    pub(crate) fn as_tile(&self) -> &TileBase {
        self.try_as_tile().expect("unreachable: unable to cast to Tile")
    }

    pub(crate) fn try_as_cowboy(&self) -> Option< &CowboyBase > {
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

    pub(crate) fn as_cowboy(&self) -> &CowboyBase {
        self.try_as_cowboy().expect("unreachable: unable to cast to Cowboy")
    }

    pub(crate) fn try_as_furnishing(&self) -> Option< &FurnishingBase > {
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

    pub(crate) fn as_furnishing(&self) -> &FurnishingBase {
        self.try_as_furnishing().expect("unreachable: unable to cast to Furnishing")
    }

    pub(crate) fn try_as_bottle(&self) -> Option< &BottleBase > {
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

    pub(crate) fn as_bottle(&self) -> &BottleBase {
        self.try_as_bottle().expect("unreachable: unable to cast to Bottle")
    }

    pub(crate) fn try_as_young_gun(&self) -> Option< &YoungGunBase > {
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

    pub(crate) fn as_young_gun(&self) -> &YoungGunBase {
        self.try_as_young_gun().expect("unreachable: unable to cast to YoungGun")
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
pub(crate) struct CowboyInner {
    pub(crate) cowboy: CowboyBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct FurnishingInner {
    pub(crate) furnishing: FurnishingBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct BottleInner {
    pub(crate) bottle: BottleBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct YoungGunInner {
    pub(crate) young_gun: YoungGunBase,
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
    pub(crate) score: i64,
    pub(crate) rowdiness: i64,
    pub(crate) kills: i64,
    pub(crate) cowboys: List<Cowboy>,
    pub(crate) young_gun: YoungGun,
    pub(crate) siesta: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct TileBase {
    pub(crate) x: i64,
    pub(crate) y: i64,
    pub(crate) tile_north: Option<Tile>,
    pub(crate) tile_east: Option<Tile>,
    pub(crate) tile_south: Option<Tile>,
    pub(crate) tile_west: Option<Tile>,
    pub(crate) cowboy: Option<Cowboy>,
    pub(crate) furnishing: Option<Furnishing>,
    pub(crate) is_balcony: bool,
    pub(crate) has_hazard: bool,
    pub(crate) bottle: Option<Bottle>,
    pub(crate) young_gun: Option<YoungGun>,
}

#[derive(Debug, Clone)]
pub(crate) struct CowboyBase {
    pub(crate) health: i64,
    pub(crate) owner: Player,
    pub(crate) is_dead: bool,
    pub(crate) job: Str,
    pub(crate) can_move: bool,
    pub(crate) tile: Option<Tile>,
    pub(crate) focus: i64,
    pub(crate) is_drunk: bool,
    pub(crate) drunk_direction: Str,
    pub(crate) tolerance: i64,
    pub(crate) turns_busy: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct FurnishingBase {
    pub(crate) is_piano: bool,
    pub(crate) tile: Option<Tile>,
    pub(crate) health: i64,
    pub(crate) is_destroyed: bool,
    pub(crate) is_playing: bool,
}

#[derive(Debug, Clone)]
pub(crate) struct BottleBase {
    pub(crate) tile: Option<Tile>,
    pub(crate) direction: Str,
    pub(crate) is_destroyed: bool,
    pub(crate) drunk_direction: Str,
}

#[derive(Debug, Clone)]
pub(crate) struct YoungGunBase {
    pub(crate) owner: Player,
    pub(crate) tile: Tile,
    pub(crate) can_call_in: bool,
    pub(crate) call_in_tile: Tile,
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
    pub(crate) cowboys: List<Cowboy>,
    pub(crate) furnishings: List<Furnishing>,
    pub(crate) jobs: List<Str>,
    pub(crate) bottles: List<Bottle>,
    pub(crate) rowdiness_to_siesta: i64,
    pub(crate) siesta_length: i64,
    pub(crate) max_cowboys_per_job: i64,
    pub(crate) sharpshooter_damage: i64,
    pub(crate) brawler_damage: i64,
    pub(crate) turns_drunk: i64,
    pub(crate) bartender_cooldown: i64,
}
