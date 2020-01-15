//! Send hordes of the undead at your opponent while defending yourself against theirs to win.

mod ai;
mod game;
mod game_object;
mod player;
mod tile;
mod tower;
mod tower_job;
mod unit;
mod unit_job;

pub use ai::AI;
pub use game::Game;
pub use game_object::GameObject;
pub use player::Player;
pub use tile::Tile;
pub use tower::Tower;
pub use tower_job::TowerJob;
pub use unit::Unit;
pub use unit_job::UnitJob;

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
                x if x == TypeId::of::<GameObject>() => obj
                    .downcast_ref::<GameObject>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<Player>() => obj
                    .downcast_ref::<Player>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<Tile>() => obj
                    .downcast_ref::<Tile>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<Tower>() => obj
                    .downcast_ref::<Tower>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<TowerJob>() => obj
                    .downcast_ref::<TowerJob>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<Unit>() => obj
                    .downcast_ref::<Unit>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<UnitJob>() => obj
                    .downcast_ref::<UnitJob>()
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
    game_object: Option<Arc<Mutex<game_object::GameObjectBase>>>,
    player: Option<Arc<Mutex<player::PlayerBase>>>,
    tile: Option<Arc<Mutex<tile::TileBase>>>,
    tower: Option<Arc<Mutex<tower::TowerBase>>>,
    tower_job: Option<Arc<Mutex<tower_job::TowerJobBase>>>,
    unit: Option<Arc<Mutex<unit::UnitBase>>>,
    unit_job: Option<Arc<Mutex<unit_job::UnitJobBase>>>,
}
