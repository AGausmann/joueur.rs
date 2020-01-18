//! There's an infestation of enemy spiders challenging your queen broodmother spider! Protect her
//! and attack the other broodmother in this turn based, node based, game.

mod inner;

mod game_object;
mod player;
mod nest;
mod web;
mod spider;
mod brood_mother;
mod spiderling;
mod spitter;
mod weaver;
mod cutter;
mod game;

pub use game_object::GameObject;
pub use player::Player;
pub use nest::Nest;
pub use web::Web;
pub use spider::Spider;
pub use brood_mother::BroodMother;
pub use spiderling::Spiderling;
pub use spitter::Spitter;
pub use weaver::Weaver;
pub use cutter::Cutter;
pub use game::Game;
