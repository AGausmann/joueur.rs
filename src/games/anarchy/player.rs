#![allow(dead_code, unused_imports)]

use super::*;
use crate::util::*;

/// A player in this game. Every AI controls one player.
pub struct Player {
}

impl Player {

    /// The name of the player.
    pub fn name(&self) -> String {
        unimplemented!()
    }

    /// What type of client this is, e.g. 'Python', 'JavaScript', or some other language. For
    /// potential data mining purposes.
    pub fn client_type(&self) -> String {
        unimplemented!()
    }

    /// If the player won the game or not.
    pub fn won(&self) -> bool {
        unimplemented!()
    }

    /// If the player lost the game or not.
    pub fn lost(&self) -> bool {
        unimplemented!()
    }

    /// The reason why the player won the game.
    pub fn reason_won(&self) -> String {
        unimplemented!()
    }

    /// The reason why the player lost the game.
    pub fn reason_lost(&self) -> String {
        unimplemented!()
    }

    /// The amount of time (in ns) remaining for this AI to send commands.
    pub fn time_remaining(&self) -> f64 {
        unimplemented!()
    }

    /// This player's opponent in the game.
    pub fn opponent(&self) -> Player {
        unimplemented!()
    }

    /// How many bribes this player has remaining to use during their turn. Each action a Building
    /// does costs 1 bribe. Any unused bribes are lost at the end of the player's turn.
    pub fn bribes_remaining(&self) -> isize {
        unimplemented!()
    }

    /// The Warehouse that serves as this player's headquarters and has extra health. If this gets
    /// destroyed they lose.
    pub fn headquarters(&self) -> Warehouse {
        unimplemented!()
    }

    /// All the buildings owned by this player.
    pub fn buildings(&self) -> List<Building> {
        unimplemented!()
    }

    /// All the warehouses owned by this player. Includes the Headquarters.
    pub fn warehouses(&self) -> List<Warehouse> {
        unimplemented!()
    }

    /// All the FireDepartments owned by this player.
    pub fn fire_departments(&self) -> List<FireDepartment> {
        unimplemented!()
    }

    /// All the PoliceDepartments owned by this player.
    pub fn police_departments(&self) -> List<PoliceDepartment> {
        unimplemented!()
    }

    /// All the WeatherStations owned by this player.
    pub fn weather_stations(&self) -> List<WeatherStation> {
        unimplemented!()
    }

    /// _Inherited from GameObject_
    ///
    /// A unique id for each instance of a GameObject or a sub class. Used for client and server
    /// communication. Should never change value after being set.
    pub fn id(&self) -> String {
        unimplemented!()
    }

    /// _Inherited from GameObject_
    ///
    /// String representing the top level Class that this game object is an instance of. Used for
    /// reflection to create new instances on clients, but exposed for convenience should AIs want
    /// this data.
    pub fn game_object_name(&self) -> String {
        unimplemented!()
    }

    /// _Inherited from GameObject_
    ///
    /// Any strings logged will be stored here. Intended for debugging.
    pub fn logs(&self) -> List<String> {
        unimplemented!()
    }

    /// _Inherited from GameObject_
    ///
    /// Adds a message to this GameObject's logs. Intended for your own debugging purposes, as
    /// strings stored here are saved in the gamelog.
    ///
    /// # Arguments
    ///
    /// - _message_ - A string to add to this GameObject's log. Intended for debugging.
    pub fn log(
        &self,
        _message: &String,
    )
    {
        unimplemented!()
    }

    /// Attempts to cast this object into an object of another class.
    ///
    /// # Errors
    ///
    /// This method will return `None` if this object cannot be casted into the target class. This
    /// happens when the base class of this object does not inherit from the target class.
    pub fn try_cast<T>(&self) -> Option<T> {
        unimplemented!()
    }

    /// Attempts to cast this object into an object of another class.
    ///
    /// # Panics
    ///
    /// Panics if the base class of this object does not inherit from the target class.
    pub fn cast<T>(&self) -> T {
        self.try_cast().unwrap()
    }
}
