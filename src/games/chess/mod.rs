//! The traditional 8x8 chess board with pieces.

mod inner;

mod game_object;
mod player;
mod game;

pub use game_object::GameObject;
pub use player::Player;
pub use game::Game;
