#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;
use crate::error::Error;

/// A player in this game. Every AI controls one player.
#[derive(Debug, Clone)]
pub struct Player {
}

impl Player {

    /// The name of the player.
    pub fn name(&self) -> Str {
        unimplemented!()
    }

    /// What type of client this is, e.g. 'Python', 'JavaScript', or some other language. For
    /// potential data mining purposes.
    pub fn client_type(&self) -> Str {
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
    pub fn reason_won(&self) -> Str {
        unimplemented!()
    }

    /// The reason why the player lost the game.
    pub fn reason_lost(&self) -> Str {
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

    /// Every Tower owned by this player.
    pub fn towers(&self) -> List<Tower> {
        unimplemented!()
    }

    /// The amount of gold this Player has.
    pub fn gold(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of mana this player has.
    pub fn mana(&self) -> i64 {
        unimplemented!()
    }

    /// The amount of health remaining for this player's main unit.
    pub fn health(&self) -> i64 {
        unimplemented!()
    }

    /// All tiles that this player can build on and move workers on.
    pub fn side(&self) -> List<Tile> {
        unimplemented!()
    }

    /// The tile that the home base is located on.
    pub fn home_base(&self) -> List<Tile> {
        unimplemented!()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// A unique id for each instance of a GameObject or a sub class. Used for client and server
    /// communication. Should never change value after being set.
    pub fn id(&self) -> Str {
        unimplemented!()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// String representing the top level Class that this game object is an instance of. Used for
    /// reflection to create new instances on clients, but exposed for convenience should AIs want
    /// this data.
    pub fn game_object_name(&self) -> Str {
        unimplemented!()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// Any strings logged will be stored here. Intended for debugging.
    pub fn logs(&self) -> List<Str> {
        unimplemented!()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// Adds a message to this GameObject's logs. Intended for your own debugging purposes, as
    /// strings stored here are saved in the gamelog.
    ///
    /// # Arguments
    ///
    /// - _message_ - A string to add to this GameObject's log. Intended for debugging.
    pub fn log(
        &self,
        _message: &str,
    )
        -> Result<(), Error>
    {
        unimplemented!()
    }
}
