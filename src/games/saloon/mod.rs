//! Use cowboys to have a good time and play some music on a Piano, while brawling with enemy
//! Cowboys.

mod inner;

mod bottle;
mod cowboy;
mod furnishing;
mod game_object;
mod player;
mod tile;
mod young_gun;
mod game;

pub use bottle::Bottle;
pub use cowboy::Cowboy;
pub use furnishing::Furnishing;
pub use game_object::GameObject;
pub use player::Player;
pub use tile::Tile;
pub use young_gun::YoungGun;
pub use game::Game;

pub use inner::Object;
