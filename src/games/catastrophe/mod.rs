//! Convert as many humans to as you can to survive in this post-apocalyptic wasteland.

mod inner;

mod game_object;
mod player;
mod tile;
mod structure;
mod unit;
mod job;
mod game;

pub use game_object::GameObject;
pub use player::Player;
pub use tile::Tile;
pub use structure::Structure;
pub use unit::Unit;
pub use job::Job;
pub use game::Game;

pub use inner::Object;
