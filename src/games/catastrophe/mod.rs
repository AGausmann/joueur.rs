//! Convert as many humans to as you can to survive in this post-apocalyptic wasteland.

mod inner;

mod game_object;
mod job;
mod player;
mod structure;
mod tile;
mod unit;
mod game;

pub use game_object::GameObject;
pub use job::Job;
pub use player::Player;
pub use structure::Structure;
pub use tile::Tile;
pub use unit::Unit;
pub use game::Game;

pub use inner::Object;
