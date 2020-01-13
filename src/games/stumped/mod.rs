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
