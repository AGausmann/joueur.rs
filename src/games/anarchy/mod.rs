//! Two player grid based game where each player tries to burn down the other player's buildings.
//! Let it burn.

mod ai;
mod building;
mod fire_department;
mod forecast;
mod game;
mod game_object;
mod player;
mod police_department;
mod warehouse;
mod weather_station;

pub use ai::AI;
pub use building::Building;
pub use fire_department::FireDepartment;
pub use forecast::Forecast;
pub use game::Game;
pub use game_object::GameObject;
pub use player::Player;
pub use police_department::PoliceDepartment;
pub use warehouse::Warehouse;
pub use weather_station::WeatherStation;

#[derive(Debug)]
struct Context {}

impl Context {
    fn try_get_obj<T>(&self, id: &str) -> Option<T> {
        unimplemented!()
    }

    fn get_obj<T>(&self, id: &str) -> T {
        self.try_get_obj(id).expect("Object is not of given type")
    }
}
