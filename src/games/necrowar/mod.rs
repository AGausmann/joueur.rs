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
