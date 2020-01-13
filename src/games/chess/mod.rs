//! The traditional 8x8 chess board with pieces.

mod ai;
mod game;
mod game_object;
mod player;

pub use ai::AI;
pub use game::Game;
pub use game_object::GameObject;
pub use player::Player;

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
