//! Gather branches and build up your lodge as beavers fight to survive.

mod inner;

mod beaver;
mod game_object;
mod job;
mod player;
mod spawner;
mod tile;
mod game;

pub use beaver::Beaver;
pub use game_object::GameObject;
pub use job::Job;
pub use player::Player;
pub use spawner::Spawner;
pub use tile::Tile;
pub use game::Game;

pub use inner::Object;
