//! Two player grid based game where each player tries to burn down the other player's buildings.
//! Let it burn.

mod inner;

mod game_object;
mod player;
mod building;
mod warehouse;
mod fire_department;
mod weather_station;
mod police_department;
mod forecast;
mod game;

pub use game_object::GameObject;
pub use player::Player;
pub use building::Building;
pub use warehouse::Warehouse;
pub use fire_department::FireDepartment;
pub use weather_station::WeatherStation;
pub use police_department::PoliceDepartment;
pub use forecast::Forecast;
pub use game::Game;
