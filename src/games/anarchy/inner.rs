#![allow(unused_imports, dead_code)]

use super::*;
use crate::types::*;
use crate::error::Error;

#[derive(Debug, Clone)]
pub(crate) struct Context {
    game: GameBase,
}

impl Context {
    pub(crate) fn run<A, R>(&mut self, _caller: &str, _function_name: &str, _args: A) -> Result<R, Error> {
        unimplemented!()
    }
}

#[derive(Debug, Clone)]
pub(crate) enum GameObject {
    GameObject(GameObjectInner),
    Player(PlayerInner),
    Building(BuildingInner),
    Warehouse(WarehouseInner),
    FireDepartment(FireDepartmentInner),
    WeatherStation(WeatherStationInner),
    PoliceDepartment(PoliceDepartmentInner),
    Forecast(ForecastInner),
}

impl GameObject {
    pub(crate) fn id(&self) -> Str {
        self.as_game_object().id.clone()
    }

    pub(crate) fn object_type(&self) -> Str {
        self.as_game_object().game_object_name.clone()
    }

    pub(crate) fn try_as_game_object(&self) -> Option< &GameObjectBase > {
        match self {
            GameObject::GameObject(obj) => Some(&obj.game_object),
            GameObject::Player(obj) => Some(&obj.game_object),
            GameObject::Building(obj) => Some(&obj.game_object),
            GameObject::Warehouse(obj) => Some(&obj.game_object),
            GameObject::FireDepartment(obj) => Some(&obj.game_object),
            GameObject::WeatherStation(obj) => Some(&obj.game_object),
            GameObject::PoliceDepartment(obj) => Some(&obj.game_object),
            GameObject::Forecast(obj) => Some(&obj.game_object),
        }
    }

    pub(crate) fn as_game_object(&self) -> &GameObjectBase {
        self.try_as_game_object().expect("unreachable: unable to cast to GameObject")
    }

    pub(crate) fn try_as_player(&self) -> Option< &PlayerBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(obj) => Some(&obj.player),
            GameObject::Building(_obj) => None,
            GameObject::Warehouse(_obj) => None,
            GameObject::FireDepartment(_obj) => None,
            GameObject::WeatherStation(_obj) => None,
            GameObject::PoliceDepartment(_obj) => None,
            GameObject::Forecast(_obj) => None,
        }
    }

    pub(crate) fn as_player(&self) -> &PlayerBase {
        self.try_as_player().expect("unreachable: unable to cast to Player")
    }

    pub(crate) fn try_as_building(&self) -> Option< &BuildingBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Building(obj) => Some(&obj.building),
            GameObject::Warehouse(obj) => Some(&obj.building),
            GameObject::FireDepartment(obj) => Some(&obj.building),
            GameObject::WeatherStation(obj) => Some(&obj.building),
            GameObject::PoliceDepartment(obj) => Some(&obj.building),
            GameObject::Forecast(_obj) => None,
        }
    }

    pub(crate) fn as_building(&self) -> &BuildingBase {
        self.try_as_building().expect("unreachable: unable to cast to Building")
    }

    pub(crate) fn try_as_warehouse(&self) -> Option< &WarehouseBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Building(_obj) => None,
            GameObject::Warehouse(obj) => Some(&obj.warehouse),
            GameObject::FireDepartment(_obj) => None,
            GameObject::WeatherStation(_obj) => None,
            GameObject::PoliceDepartment(_obj) => None,
            GameObject::Forecast(_obj) => None,
        }
    }

    pub(crate) fn as_warehouse(&self) -> &WarehouseBase {
        self.try_as_warehouse().expect("unreachable: unable to cast to Warehouse")
    }

    pub(crate) fn try_as_fire_department(&self) -> Option< &FireDepartmentBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Building(_obj) => None,
            GameObject::Warehouse(_obj) => None,
            GameObject::FireDepartment(obj) => Some(&obj.fire_department),
            GameObject::WeatherStation(_obj) => None,
            GameObject::PoliceDepartment(_obj) => None,
            GameObject::Forecast(_obj) => None,
        }
    }

    pub(crate) fn as_fire_department(&self) -> &FireDepartmentBase {
        self.try_as_fire_department().expect("unreachable: unable to cast to FireDepartment")
    }

    pub(crate) fn try_as_weather_station(&self) -> Option< &WeatherStationBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Building(_obj) => None,
            GameObject::Warehouse(_obj) => None,
            GameObject::FireDepartment(_obj) => None,
            GameObject::WeatherStation(obj) => Some(&obj.weather_station),
            GameObject::PoliceDepartment(_obj) => None,
            GameObject::Forecast(_obj) => None,
        }
    }

    pub(crate) fn as_weather_station(&self) -> &WeatherStationBase {
        self.try_as_weather_station().expect("unreachable: unable to cast to WeatherStation")
    }

    pub(crate) fn try_as_police_department(&self) -> Option< &PoliceDepartmentBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Building(_obj) => None,
            GameObject::Warehouse(_obj) => None,
            GameObject::FireDepartment(_obj) => None,
            GameObject::WeatherStation(_obj) => None,
            GameObject::PoliceDepartment(obj) => Some(&obj.police_department),
            GameObject::Forecast(_obj) => None,
        }
    }

    pub(crate) fn as_police_department(&self) -> &PoliceDepartmentBase {
        self.try_as_police_department().expect("unreachable: unable to cast to PoliceDepartment")
    }

    pub(crate) fn try_as_forecast(&self) -> Option< &ForecastBase > {
        match self {
            GameObject::GameObject(_obj) => None,
            GameObject::Player(_obj) => None,
            GameObject::Building(_obj) => None,
            GameObject::Warehouse(_obj) => None,
            GameObject::FireDepartment(_obj) => None,
            GameObject::WeatherStation(_obj) => None,
            GameObject::PoliceDepartment(_obj) => None,
            GameObject::Forecast(obj) => Some(&obj.forecast),
        }
    }

    pub(crate) fn as_forecast(&self) -> &ForecastBase {
        self.try_as_forecast().expect("unreachable: unable to cast to Forecast")
    }
}

#[derive(Debug, Clone)]
pub(crate) struct GameObjectInner {
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct PlayerInner {
    pub(crate) player: PlayerBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct BuildingInner {
    pub(crate) building: BuildingBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct WarehouseInner {
    pub(crate) warehouse: WarehouseBase,
    pub(crate) building: BuildingBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct FireDepartmentInner {
    pub(crate) fire_department: FireDepartmentBase,
    pub(crate) building: BuildingBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct WeatherStationInner {
    pub(crate) weather_station: WeatherStationBase,
    pub(crate) building: BuildingBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct PoliceDepartmentInner {
    pub(crate) police_department: PoliceDepartmentBase,
    pub(crate) building: BuildingBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct ForecastInner {
    pub(crate) forecast: ForecastBase,
    pub(crate) game_object: GameObjectBase,
}

#[derive(Debug, Clone)]
pub(crate) struct GameObjectBase {
    pub(crate) id: Str,
    pub(crate) game_object_name: Str,
    pub(crate) logs: List<Str>,
}

#[derive(Debug, Clone)]
pub(crate) struct PlayerBase {
    pub(crate) name: Str,
    pub(crate) client_type: Str,
    pub(crate) won: bool,
    pub(crate) lost: bool,
    pub(crate) reason_won: Str,
    pub(crate) reason_lost: Str,
    pub(crate) time_remaining: f64,
    pub(crate) opponent: Player,
    pub(crate) bribes_remaining: i64,
    pub(crate) headquarters: Warehouse,
    pub(crate) buildings: List<Building>,
    pub(crate) warehouses: List<Warehouse>,
    pub(crate) fire_departments: List<FireDepartment>,
    pub(crate) police_departments: List<PoliceDepartment>,
    pub(crate) weather_stations: List<WeatherStation>,
}

#[derive(Debug, Clone)]
pub(crate) struct BuildingBase {
    pub(crate) health: i64,
    pub(crate) owner: Player,
    pub(crate) is_headquarters: bool,
    pub(crate) bribed: bool,
    pub(crate) x: i64,
    pub(crate) y: i64,
    pub(crate) fire: i64,
    pub(crate) building_north: Option<Building>,
    pub(crate) building_east: Option<Building>,
    pub(crate) building_south: Option<Building>,
    pub(crate) building_west: Option<Building>,
}

#[derive(Debug, Clone)]
pub(crate) struct WarehouseBase {
    pub(crate) fire_added: i64,
    pub(crate) exposure: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct FireDepartmentBase {
    pub(crate) fire_extinguished: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct WeatherStationBase {
}

#[derive(Debug, Clone)]
pub(crate) struct PoliceDepartmentBase {
}

#[derive(Debug, Clone)]
pub(crate) struct ForecastBase {
    pub(crate) direction: Str,
    pub(crate) intensity: i64,
    pub(crate) controlling_player: Player,
}

#[derive(Debug, Clone)]
pub(crate) struct GameBase {
    pub(crate) game_objects: Map<Str, GameObject>,
    pub(crate) players: List<Player>,
    pub(crate) session: Str,
    pub(crate) current_player: Player,
    pub(crate) current_turn: i64,
    pub(crate) max_turns: i64,
    pub(crate) time_added_per_turn: i64,
    pub(crate) map_width: i64,
    pub(crate) map_height: i64,
    pub(crate) buildings: List<Building>,
    pub(crate) forecasts: List<Forecast>,
    pub(crate) current_forecast: Forecast,
    pub(crate) next_forecast: Option<Forecast>,
    pub(crate) base_bribes_per_turn: i64,
    pub(crate) max_fire: i64,
    pub(crate) max_forecast_intensity: i64,
}
