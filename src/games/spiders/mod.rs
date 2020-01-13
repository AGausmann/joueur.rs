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

use crate::error::Error;

#[derive(Debug)]
struct Context {}

impl Context {
    fn try_get_obj<T>(&self, _id: &str) -> Option<T> {
        unimplemented!()
    }

    fn get_obj<T>(&self, id: &str) -> T {
        self.try_get_obj(id).expect("Object is not of given type")
    }

    fn run<T, R>(&self, _caller: &str, _function_name: &str, _args: T) -> Result<R, Error> {
        unimplemented!()
    }
}
