//! Collect of the most of the rarest mineral orbiting aroung the sun and outcompete your
//! competetor.

mod inner;

mod game_object;
mod player;
mod body;
mod projectile;
mod unit;
mod job;
mod game;

pub use game_object::GameObject;
pub use player::Player;
pub use body::Body;
pub use projectile::Projectile;
pub use unit::Unit;
pub use job::Job;
pub use game::Game;
