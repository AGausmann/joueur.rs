//! The simple version of American Checkers. An 8x8 board with 12 checkers on each side that must
//! move diagonally to the opposing side until kinged.

mod inner;

mod game_object;
mod player;
mod checker;
mod game;

pub use game_object::GameObject;
pub use player::Player;
pub use checker::Checker;
pub use game::Game;
