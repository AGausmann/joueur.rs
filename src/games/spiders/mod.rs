//! There's an infestation of enemy spiders challenging your queen broodmother spider! Protect her
//! and attack the other broodmother in this turn based, node based, game.

mod ai;
mod brood_mother;
mod cutter;
mod game;
mod game_object;
mod nest;
mod player;
mod spider;
mod spiderling;
mod spitter;
mod weaver;
mod web;

pub use ai::AI;
pub use brood_mother::BroodMother;
pub use cutter::Cutter;
pub use game::Game;
pub use game_object::GameObject;
pub use nest::Nest;
pub use player::Player;
pub use spider::Spider;
pub use spiderling::Spiderling;
pub use spitter::Spitter;
pub use weaver::Weaver;
pub use web::Web;

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
                x if x == TypeId::of::<BroodMother>() => obj
                    .downcast_ref::<BroodMother>()
                    .and_then(|base| base.try_upcast()),
                x if x == TypeId::of::<Cutter>() => obj
                    .downcast_ref::<Cutter>()
                    .and_then(|base| base.try_upcast()),
                x if x == TypeId::of::<GameObject>() => obj
                    .downcast_ref::<GameObject>()
                    .and_then(|base| base.try_upcast()),
                x if x == TypeId::of::<Nest>() => obj
                    .downcast_ref::<Nest>()
                    .and_then(|base| base.try_upcast()),
                x if x == TypeId::of::<Player>() => obj
                    .downcast_ref::<Player>()
                    .and_then(|base| base.try_upcast()),
                x if x == TypeId::of::<Spider>() => obj
                    .downcast_ref::<Spider>()
                    .and_then(|base| base.try_upcast()),
                x if x == TypeId::of::<Spiderling>() => obj
                    .downcast_ref::<Spiderling>()
                    .and_then(|base| base.try_upcast()),
                x if x == TypeId::of::<Spitter>() => obj
                    .downcast_ref::<Spitter>()
                    .and_then(|base| base.try_upcast()),
                x if x == TypeId::of::<Weaver>() => obj
                    .downcast_ref::<Weaver>()
                    .and_then(|base| base.try_upcast()),
                x if x == TypeId::of::<Web>() => obj
                    .downcast_ref::<Web>()
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
