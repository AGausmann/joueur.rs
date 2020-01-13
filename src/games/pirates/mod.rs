//! Steal from merchants and become the most infamous pirate.

mod ai;
mod game;
mod game_object;
mod player;
mod port;
mod tile;
mod unit;

pub use ai::AI;
pub use game::Game;
pub use game_object::GameObject;
pub use player::Player;
pub use port::Port;
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
