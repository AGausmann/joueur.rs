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

use std::any::{Any, TypeId};
use std::sync::{Arc, Mutex, Weak};

use crate::error::Error;
use crate::types::*;

#[doc(hidden)]
#[derive(Debug)]
pub struct Context {
    game_objects: Map<Str, Box<dyn Any>>,
}

impl Context {
    fn try_get_obj<T: Object>(&self, id: &str) -> Option<T> {
        self.game_objects.get(id)
            .and_then(|obj| match obj.type_id() {
                x if x == TypeId::of::<Building>() => obj
                    .downcast_ref::<Building>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<FireDepartment>() => obj
                    .downcast_ref::<FireDepartment>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<Forecast>() => obj
                    .downcast_ref::<Forecast>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<GameObject>() => obj
                    .downcast_ref::<GameObject>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<Player>() => obj
                    .downcast_ref::<Player>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<PoliceDepartment>() => obj
                    .downcast_ref::<PoliceDepartment>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<Warehouse>() => obj
                    .downcast_ref::<Warehouse>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                x if x == TypeId::of::<WeatherStation>() => obj
                    .downcast_ref::<WeatherStation>()
                    .and_then(|base| T::from_bases(base.to_bases())),
                _ => panic!("unknown game object type"),
            })
    }

    fn get_obj<T: Object>(&self, id: &str) -> T {
        self.try_get_obj(id).expect("Object is not of given type")
    }

    fn run<T, R>(&self, _caller: &str, _function_name: &str, _args: T) -> Result<R, Error> {
        unimplemented!()
    }
}

pub trait Object: ObjectInner  {}

mod inner {
    use super::*;

    pub trait ObjectInner: Any + Sized {
        fn to_bases(&self) -> Bases;

        fn from_bases(bases: Bases) -> Option<Self>;
    }
}

use inner::ObjectInner;

#[doc(hidden)]
#[derive(Debug, Default)]
pub struct Bases {
    context: Option<Weak<Context>>,
    id: Option<Str>,
    building: Option<Arc<Mutex<building::BuildingBase>>>,
    fire_department: Option<Arc<Mutex<fire_department::FireDepartmentBase>>>,
    forecast: Option<Arc<Mutex<forecast::ForecastBase>>>,
    game_object: Option<Arc<Mutex<game_object::GameObjectBase>>>,
    player: Option<Arc<Mutex<player::PlayerBase>>>,
    police_department: Option<Arc<Mutex<police_department::PoliceDepartmentBase>>>,
    warehouse: Option<Arc<Mutex<warehouse::WarehouseBase>>>,
    weather_station: Option<Arc<Mutex<weather_station::WeatherStationBase>>>,
}
