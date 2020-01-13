//! Convert as many humans to as you can to survive in this post-apocalyptic wasteland.

mod ai;
mod game;
mod game_object;
mod job;
mod player;
mod structure;
mod tile;
mod unit;

pub use ai::AI;
pub use game::Game;
pub use game_object::GameObject;
pub use job::Job;
pub use player::Player;
pub use structure::Structure;
pub use tile::Tile;
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
