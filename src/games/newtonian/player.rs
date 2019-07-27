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

    /// Every Unit owned by this Player.
    pub fn units(&self) -> List<Unit> {
        unimplemented!()
    }

    /// The amount of heat this Player has.
    pub fn heat(&self) -> isize {
        unimplemented!()
    }

    /// The amount of pressure this Player has.
    pub fn pressure(&self) -> isize {
        unimplemented!()
    }

    /// The time left till a intern spawns. (0 to spawnTime).
    pub fn intern_spawn(&self) -> isize {
        unimplemented!()
    }

    /// The time left till a physicist spawns. (0 to spawnTime).
    pub fn physicist_spawn(&self) -> isize {
        unimplemented!()
    }

    /// The time left till a manager spawns. (0 to spawnTime).
    pub fn manager_spawn(&self) -> isize {
        unimplemented!()
    }

    /// All the tiles this Player's units can spawn on. (listed from the outer edges inward, from
    /// top to bottom).
    pub fn spawn_tiles(&self) -> List<Tile> {
        unimplemented!()
    }

    /// Every generator Tile owned by this Player. (listed from the outer edges inward, from top to
    /// bottom).
    pub fn generator_tiles(&self) -> List<Tile> {
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
