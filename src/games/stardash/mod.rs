//! Collect of the most of the rarest mineral orbiting aroung the sun and outcompete your
//! competetor.

mod ai;
mod body;
mod game;
mod game_object;
mod job;
mod player;
mod projectile;
mod unit;

pub use ai::AI;
pub use body::Body;
pub use game::Game;
pub use game_object::GameObject;
pub use job::Job;
pub use player::Player;
pub use projectile::Projectile;
pub use unit::Unit;

#[derive(Debug)]
struct Context {}

impl Context {
    fn try_get_obj<T>(&self, id: &str) -> Option<T> {
        unimplemented!()
    }

    fn get_obj<T>(&self, id: &str) -> T {
        self.try_get_obj(id).expect("Object is not of given type")
    }
}
