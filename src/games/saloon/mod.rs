//! Use cowboys to have a good time and play some music on a Piano, while brawling with enemy
//! Cowboys.

mod ai;
mod bottle;
mod cowboy;
mod furnishing;
mod game;
mod game_object;
mod player;
mod tile;
mod young_gun;

pub use ai::AI;
pub use bottle::Bottle;
pub use cowboy::Cowboy;
pub use furnishing::Furnishing;
pub use game::Game;
pub use game_object::GameObject;
pub use player::Player;
pub use tile::Tile;
pub use young_gun::YoungGun;

use std::any::{Any, TypeId};
use std::sync::{Arc, Mutex, Weak};

use crate::error::Error;
use crate::types::*;

#[doc(hidden)]
#[derive(Debug)]
pub struct Context {
    game_objects: Map<Str, Box<dyn Any>>,
}

impl Context {
    fn try_get_obj<T: Object>(&self, id: &str) -> Option<T> {
        self.game_objects.get(id)
            .and_then(|obj| match obj.type_id() {
                x if x == TypeId::of::<Bottle>() => obj
                    .downcast_ref::<Bottle>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<Cowboy>() => obj
                    .downcast_ref::<Cowboy>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<Furnishing>() => obj
                    .downcast_ref::<Furnishing>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<GameObject>() => obj
                    .downcast_ref::<GameObject>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<Player>() => obj
                    .downcast_ref::<Player>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<Tile>() => obj
                    .downcast_ref::<Tile>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<YoungGun>() => obj
                    .downcast_ref::<YoungGun>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                _ => panic!("unknown game object type"),
            })
    }

    fn get_obj<T: Object>(&self, id: &str) -> T {
        self.try_get_obj(id).expect("Object is not of given type")
    }

    fn run<T, R>(&self, _caller: &str, _function_name: &str, _args: T) -> Result<R, Error> {
        unimplemented!()
    }
}

pub trait Object: ObjectInner  {}

mod inner {
    use super::*;

    pub trait ObjectInner: Any + Sized {
        fn to_bases(&self) -> Bases;

        fn from_bases(bases: Bases) -> Option<Self>;
    }
}

use inner::ObjectInner;

#[doc(hidden)]
#[derive(Debug, Default)]
pub struct Bases {
    context: Option<Weak<Context>>,
    id: Option<Str>,
    bottle: Option<Arc<Mutex<bottle::BottleBase>>>,
    cowboy: Option<Arc<Mutex<cowboy::CowboyBase>>>,
    furnishing: Option<Arc<Mutex<furnishing::FurnishingBase>>>,
    game_object: Option<Arc<Mutex<game_object::GameObjectBase>>>,
    player: Option<Arc<Mutex<player::PlayerBase>>>,
    tile: Option<Arc<Mutex<tile::TileBase>>>,
    young_gun: Option<Arc<Mutex<young_gun::YoungGunBase>>>,
}
