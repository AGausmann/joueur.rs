//! Use cowboys to have a good time and play some music on a Piano, while brawling with enemy
//! Cowboys.

mod ai;
mod bottle;
mod cowboy;
mod furnishing;
mod game;
mod game_object;
mod player;
mod tile;
mod young_gun;

pub use ai::AI;
pub use bottle::Bottle;
pub use cowboy::Cowboy;
pub use furnishing::Furnishing;
pub use game::Game;
pub use game_object::GameObject;
pub use player::Player;
pub use tile::Tile;
pub use young_gun::YoungGun;

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
