//! Gather branches and build up your lodge as beavers fight to survive.

mod ai;
mod beaver;
mod game;
mod game_object;
mod job;
mod player;
mod spawner;
mod tile;

pub use ai::AI;
pub use beaver::Beaver;
pub use game::Game;
pub use game_object::GameObject;
pub use job::Job;
pub use player::Player;
pub use spawner::Spawner;
pub use tile::Tile;

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
                x if x == TypeId::of::<Beaver>() => obj
                    .downcast_ref::<Beaver>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<GameObject>() => obj
                    .downcast_ref::<GameObject>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<Job>() => obj
                    .downcast_ref::<Job>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<Player>() => obj
                    .downcast_ref::<Player>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<Spawner>() => obj
                    .downcast_ref::<Spawner>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<Tile>() => obj
                    .downcast_ref::<Tile>()
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
    beaver: Option<Arc<Mutex<beaver::BeaverBase>>>,
    game_object: Option<Arc<Mutex<game_object::GameObjectBase>>>,
    job: Option<Arc<Mutex<job::JobBase>>>,
    player: Option<Arc<Mutex<player::PlayerBase>>>,
    spawner: Option<Arc<Mutex<spawner::SpawnerBase>>>,
    tile: Option<Arc<Mutex<tile::TileBase>>>,
}
