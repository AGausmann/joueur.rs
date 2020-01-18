//! Collect of the most of the rarest mineral orbiting aroung the sun and outcompete your
//! competetor.

mod inner;

mod body;
mod game_object;
mod job;
mod player;
mod projectile;
mod unit;
mod game;

pub use body::Body;
pub use game_object::GameObject;
pub use job::Job;
pub use player::Player;
pub use projectile::Projectile;
pub use unit::Unit;
pub use game::Game;

pub use inner::Object;
