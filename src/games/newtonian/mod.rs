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
