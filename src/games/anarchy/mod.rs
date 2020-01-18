//! Two player grid based game where each player tries to burn down the other player's buildings.
//! Let it burn.

mod inner;

mod building;
mod fire_department;
mod forecast;
mod game_object;
mod player;
mod police_department;
mod warehouse;
mod weather_station;
mod game;

pub use building::Building;
pub use fire_department::FireDepartment;
pub use forecast::Forecast;
pub use game_object::GameObject;
pub use player::Player;
pub use police_department::PoliceDepartment;
pub use warehouse::Warehouse;
pub use weather_station::WeatherStation;
pub use game::Game;

pub use inner::Object;
