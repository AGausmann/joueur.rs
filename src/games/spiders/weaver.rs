#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;
use crate::error::Error;

/// A Spiderling that can alter existing Webs by weaving to add or remove silk from the Webs, thus
/// altering its strength.
#[derive(Debug, Clone)]
pub struct Weaver {
}

impl Weaver {

    /// The Web that this Weaver is strengthening. None if not strengthening.
    pub fn strengthening_web(&self) -> Option<Web> {
        unimplemented!()
    }

    /// The Web that this Weaver is weakening. None if not weakening.
    pub fn weakening_web(&self) -> Option<Web> {
        unimplemented!()
    }

    /// _Inherited from [`Spiderling`]_
    ///
    /// When empty string this Spiderling is not busy, and can act. Otherwise a string representing
    /// what it is busy with, e.g. 'Moving', 'Attacking'.
    pub fn busy(&self) -> Str {
        unimplemented!()
    }

    /// _Inherited from [`Spiderling`]_
    ///
    /// How much work needs to be done for this Spiderling to finish being busy. See docs for the
    /// Work forumla.
    pub fn work_remaining(&self) -> f64 {
        unimplemented!()
    }

    /// _Inherited from [`Spiderling`]_
    ///
    /// The number of Spiderlings busy with the same work this Spiderling is doing, speeding up the
    /// task.
    pub fn number_of_coworkers(&self) -> i64 {
        unimplemented!()
    }

    /// _Inherited from [`Spiderling`]_
    ///
    /// The Web this Spiderling is using to move. None if it is not moving.
    pub fn moving_on_web(&self) -> Option<Web> {
        unimplemented!()
    }

    /// _Inherited from [`Spiderling`]_
    ///
    /// The Nest this Spiderling is moving to. None if it is not moving.
    pub fn moving_to_nest(&self) -> Option<Nest> {
        unimplemented!()
    }

    /// _Inherited from [`Spider`]_
    ///
    /// The Player that owns this Spider, and can command it.
    pub fn owner(&self) -> Player {
        unimplemented!()
    }

    /// _Inherited from [`Spider`]_
    ///
    /// The Nest that this Spider is currently on. None when moving on a Web.
    pub fn nest(&self) -> Option<Nest> {
        unimplemented!()
    }

    /// _Inherited from [`Spider`]_
    ///
    /// If this Spider is dead and has been removed from the game.
    pub fn is_dead(&self) -> bool {
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

    /// Weaves more silk into an existing Web to strengthen it.
    ///
    /// # Arguments
    ///
    /// - _web_ - The web you want to strengthen. Must be connected to the Nest this Weaver is
    /// currently on.
    ///
    /// # Returns
    ///
    /// True if the strengthen was successfully started, false otherwise.
    pub fn strengthen(
        &self,
        _web: &Web,
    )
        -> Result<bool, Error>
    {
        unimplemented!()
    }

    /// Weaves more silk into an existing Web to strengthen it.
    ///
    /// # Arguments
    ///
    /// - _web_ - The web you want to weaken. Must be connected to the Nest this Weaver is
    /// currently on.
    ///
    /// # Returns
    ///
    /// True if the weaken was successfully started, false otherwise.
    pub fn weaken(
        &self,
        _web: &Web,
    )
        -> Result<bool, Error>
    {
        unimplemented!()
    }

    /// _Inherited from [`Spiderling`]_
    ///
    /// Starts moving the Spiderling across a Web to another Nest.
    ///
    /// # Arguments
    ///
    /// - _web_ - The Web you want to move across to the other Nest.
    ///
    /// # Returns
    ///
    /// True if the move was successful, false otherwise.
    pub fn move_(
        &self,
        _web: &Web,
    )
        -> Result<bool, Error>
    {
        unimplemented!()
    }

    /// _Inherited from [`Spiderling`]_
    ///
    /// Attacks another Spiderling
    ///
    /// # Arguments
    ///
    /// - _spiderling_ - The Spiderling to attack.
    ///
    /// # Returns
    ///
    /// True if the attack was successful, false otherwise.
    pub fn attack(
        &self,
        _spiderling: &Spiderling,
    )
        -> Result<bool, Error>
    {
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
