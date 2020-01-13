//! Combine elements and be the first scientists to create fusion.

mod ai;
mod game;
mod game_object;
mod job;
mod machine;
mod player;
mod tile;
mod unit;

pub use ai::AI;
pub use game::Game;
pub use game_object::GameObject;
pub use job::Job;
pub use machine::Machine;
pub use player::Player;
pub use tile::Tile;
pub use unit::Unit;

use std::any::{Any, TypeId};
use std::sync::Weak;

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
                    .and_then(|base| base.try_upcast()),
                x if x == TypeId::of::<Job>() => obj
                    .downcast_ref::<Job>()
                    .and_then(|base| base.try_upcast()),
                x if x == TypeId::of::<Machine>() => obj
                    .downcast_ref::<Machine>()
                    .and_then(|base| base.try_upcast()),
                x if x == TypeId::of::<Player>() => obj
                    .downcast_ref::<Player>()
                    .and_then(|base| base.try_upcast()),
                x if x == TypeId::of::<Tile>() => obj
                    .downcast_ref::<Tile>()
                    .and_then(|base| base.try_upcast()),
                x if x == TypeId::of::<Unit>() => obj
                    .downcast_ref::<Unit>()
                    .and_then(|base| base.try_upcast()),
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

    pub trait ObjectInner: Any {
        fn shallow(context: Weak<Context>, id: Str) -> Self;
    }
}

use inner::ObjectInner;
