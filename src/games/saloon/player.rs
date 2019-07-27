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

    /// How many times their team has played a piano.
    pub fn score(&self) -> isize {
        unimplemented!()
    }

    /// How rowdy their team is. When it gets too high their team takes a collective siesta.
    pub fn rowdiness(&self) -> isize {
        unimplemented!()
    }

    /// How many enemy Cowboys this player's team has killed.
    pub fn kills(&self) -> isize {
        unimplemented!()
    }

    /// Every Cowboy owned by this Player.
    pub fn cowboys(&self) -> List<Cowboy> {
        unimplemented!()
    }

    /// The YoungGun this Player uses to call in new Cowboys.
    pub fn young_gun(&self) -> YoungGun {
        unimplemented!()
    }

    /// 0 when not having a team siesta. When greater than 0 represents how many turns left for the
    /// team siesta to complete.
    pub fn siesta(&self) -> isize {
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
