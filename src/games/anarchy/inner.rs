#![allow(unused_imports, dead_code)]

use std::sync::{Arc, Mutex, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

#[derive(Debug, Clone)]
pub struct Context {
    self_ref: Option<Weak<Mutex<Context>>>,
    game: Game,
}

impl Context {
    pub(crate) fn get_ref(&self) -> Weak<Mutex<Context>> {
        self.self_ref.clone().unwrap()
    }

    pub(crate) fn run<A, R>(&mut self, _caller: &str, _function_name: &str, _args: A) -> Result<R, Error> {
        unimplemented!()
    }
}

pub trait Object: ObjectInner {
}

pub trait ObjectInner: Sized {
    fn from_game_object(game_obj: &Arc<Mutex<AnyGameObject>>, context: &Weak<Mutex<Context>>) -> Option<Self>;
}

#[derive(Debug, Clone)]
pub enum AnyGameObject {
    GameObject(GameObjectInner),
    Player(PlayerInner),
    Building(BuildingInner),
    Warehouse(WarehouseInner),
    FireDepartment(FireDepartmentInner),
    WeatherStation(WeatherStationInner),
    PoliceDepartment(PoliceDepartmentInner),
    Forecast(ForecastInner),
}

impl AnyGameObject {
    pub fn id(&self) -> Str {
        self.as_game_object().id.clone()
    }

    pub fn object_type(&self) -> Str {
        self.as_game_object().game_object_name.clone()
    }

    pub fn try_as_game_object(&self) -> Option< &GameObjectBase > {
        match self {
            AnyGameObject::GameObject(obj) => Some(&obj.game_object),
            AnyGameObject::Player(obj) => Some(&obj.game_object),
            AnyGameObject::Building(obj) => Some(&obj.game_object),
            AnyGameObject::Warehouse(obj) => Some(&obj.game_object),
            AnyGameObject::FireDepartment(obj) => Some(&obj.game_object),
            AnyGameObject::WeatherStation(obj) => Some(&obj.game_object),
            AnyGameObject::PoliceDepartment(obj) => Some(&obj.game_object),
            AnyGameObject::Forecast(obj) => Some(&obj.game_object),
        }
    }

    pub fn as_game_object(&self) -> &GameObjectBase {
        self.try_as_game_object().expect("unreachable: unable to cast to GameObject")
    }

    pub fn try_as_player(&self) -> Option< &PlayerBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(obj) => Some(&obj.player),
            AnyGameObject::Building(_obj) => None,
            AnyGameObject::Warehouse(_obj) => None,
            AnyGameObject::FireDepartment(_obj) => None,
            AnyGameObject::WeatherStation(_obj) => None,
            AnyGameObject::PoliceDepartment(_obj) => None,
            AnyGameObject::Forecast(_obj) => None,
        }
    }

    pub fn as_player(&self) -> &PlayerBase {
        self.try_as_player().expect("unreachable: unable to cast to Player")
    }

    pub fn try_as_building(&self) -> Option< &BuildingBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Building(obj) => Some(&obj.building),
            AnyGameObject::Warehouse(obj) => Some(&obj.building),
            AnyGameObject::FireDepartment(obj) => Some(&obj.building),
            AnyGameObject::WeatherStation(obj) => Some(&obj.building),
            AnyGameObject::PoliceDepartment(obj) => Some(&obj.building),
            AnyGameObject::Forecast(_obj) => None,
        }
    }

    pub fn as_building(&self) -> &BuildingBase {
        self.try_as_building().expect("unreachable: unable to cast to Building")
    }

    pub fn try_as_warehouse(&self) -> Option< &WarehouseBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Building(_obj) => None,
            AnyGameObject::Warehouse(obj) => Some(&obj.warehouse),
            AnyGameObject::FireDepartment(_obj) => None,
            AnyGameObject::WeatherStation(_obj) => None,
            AnyGameObject::PoliceDepartment(_obj) => None,
            AnyGameObject::Forecast(_obj) => None,
        }
    }

    pub fn as_warehouse(&self) -> &WarehouseBase {
        self.try_as_warehouse().expect("unreachable: unable to cast to Warehouse")
    }

    pub fn try_as_fire_department(&self) -> Option< &FireDepartmentBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Building(_obj) => None,
            AnyGameObject::Warehouse(_obj) => None,
            AnyGameObject::FireDepartment(obj) => Some(&obj.fire_department),
            AnyGameObject::WeatherStation(_obj) => None,
            AnyGameObject::PoliceDepartment(_obj) => None,
            AnyGameObject::Forecast(_obj) => None,
        }
    }

    pub fn as_fire_department(&self) -> &FireDepartmentBase {
        self.try_as_fire_department().expect("unreachable: unable to cast to FireDepartment")
    }

    pub fn try_as_weather_station(&self) -> Option< &WeatherStationBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Building(_obj) => None,
            AnyGameObject::Warehouse(_obj) => None,
            AnyGameObject::FireDepartment(_obj) => None,
            AnyGameObject::WeatherStation(obj) => Some(&obj.weather_station),
            AnyGameObject::PoliceDepartment(_obj) => None,
            AnyGameObject::Forecast(_obj) => None,
        }
    }

    pub fn as_weather_station(&self) -> &WeatherStationBase {
        self.try_as_weather_station().expect("unreachable: unable to cast to WeatherStation")
    }

    pub fn try_as_police_department(&self) -> Option< &PoliceDepartmentBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Building(_obj) => None,
            AnyGameObject::Warehouse(_obj) => None,
            AnyGameObject::FireDepartment(_obj) => None,
            AnyGameObject::WeatherStation(_obj) => None,
            AnyGameObject::PoliceDepartment(obj) => Some(&obj.police_department),
            AnyGameObject::Forecast(_obj) => None,
        }
    }

    pub fn as_police_department(&self) -> &PoliceDepartmentBase {
        self.try_as_police_department().expect("unreachable: unable to cast to PoliceDepartment")
    }

    pub fn try_as_forecast(&self) -> Option< &ForecastBase > {
        match self {
            AnyGameObject::GameObject(_obj) => None,
            AnyGameObject::Player(_obj) => None,
            AnyGameObject::Building(_obj) => None,
            AnyGameObject::Warehouse(_obj) => None,
            AnyGameObject::FireDepartment(_obj) => None,
            AnyGameObject::WeatherStation(_obj) => None,
            AnyGameObject::PoliceDepartment(_obj) => None,
            AnyGameObject::Forecast(obj) => Some(&obj.forecast),
        }
    }

    pub fn as_forecast(&self) -> &ForecastBase {
        self.try_as_forecast().expect("unreachable: unable to cast to Forecast")
    }
}

#[derive(Debug, Clone)]
pub struct GameObjectInner {
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct PlayerInner {
    pub player: PlayerBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct BuildingInner {
    pub building: BuildingBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct WarehouseInner {
    pub warehouse: WarehouseBase,
    pub building: BuildingBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct FireDepartmentInner {
    pub fire_department: FireDepartmentBase,
    pub building: BuildingBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct WeatherStationInner {
    pub weather_station: WeatherStationBase,
    pub building: BuildingBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct PoliceDepartmentInner {
    pub police_department: PoliceDepartmentBase,
    pub building: BuildingBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct ForecastInner {
    pub forecast: ForecastBase,
    pub game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub struct GameObjectBase {
    pub id: Str,
    pub game_object_name: Str,
    pub logs: List<Str>,
}

#[derive(Debug, Clone)]
pub struct PlayerBase {
    pub name: Str,
    pub client_type: Str,
    pub won: bool,
    pub lost: bool,
    pub reason_won: Str,
    pub reason_lost: Str,
    pub time_remaining: f64,
    pub opponent: Player,
    pub bribes_remaining: i64,
    pub headquarters: Warehouse,
    pub buildings: List<Building>,
    pub warehouses: List<Warehouse>,
    pub fire_departments: List<FireDepartment>,
    pub police_departments: List<PoliceDepartment>,
    pub weather_stations: List<WeatherStation>,
}

#[derive(Debug, Clone)]
pub struct BuildingBase {
    pub health: i64,
    pub owner: Player,
    pub is_headquarters: bool,
    pub bribed: bool,
    pub x: i64,
    pub y: i64,
    pub fire: i64,
    pub building_north: Option<Building>,
    pub building_east: Option<Building>,
    pub building_south: Option<Building>,
    pub building_west: Option<Building>,
}

#[derive(Debug, Clone)]
pub struct WarehouseBase {
    pub fire_added: i64,
    pub exposure: i64,
}

#[derive(Debug, Clone)]
pub struct FireDepartmentBase {
    pub fire_extinguished: i64,
}

#[derive(Debug, Clone)]
pub struct WeatherStationBase {
}

#[derive(Debug, Clone)]
pub struct PoliceDepartmentBase {
}

#[derive(Debug, Clone)]
pub struct ForecastBase {
    pub direction: Str,
    pub intensity: i64,
    pub controlling_player: Player,
}

#[derive(Debug, Clone)]
pub struct GameBase {
    pub game_objects: Map<Str, GameObject>,
    pub players: List<Player>,
    pub session: Str,
    pub current_player: Player,
    pub current_turn: i64,
    pub max_turns: i64,
    pub time_added_per_turn: i64,
    pub map_width: i64,
    pub map_height: i64,
    pub buildings: List<Building>,
    pub forecasts: List<Forecast>,
    pub current_forecast: Forecast,
    pub next_forecast: Option<Forecast>,
    pub base_bribes_per_turn: i64,
    pub max_fire: i64,
    pub max_forecast_intensity: i64,
}
