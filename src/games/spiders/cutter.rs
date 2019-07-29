#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;

/// A Spiderling that can cut existing Webs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cutter {
}

impl Cutter {

    /// The Web that this Cutter is trying to cut. None if not cutting.
    pub fn cutting_web(&self) -> Option<Web> {
        unimplemented!()
    }

    /// _Inherited from Spiderling_
    ///
    /// When empty string this Spiderling is not busy, and can act. Otherwise a string representing
    /// what it is busy with, e.g. 'Moving', 'Attacking'.
    pub fn busy(&self) -> Str {
        unimplemented!()
    }

    /// _Inherited from Spiderling_
    ///
    /// How much work needs to be done for this Spiderling to finish being busy. See docs for the
    /// Work forumla.
    pub fn work_remaining(&self) -> f64 {
        unimplemented!()
    }

    /// _Inherited from Spiderling_
    ///
    /// The number of Spiderlings busy with the same work this Spiderling is doing, speeding up the
    /// task.
    pub fn number_of_coworkers(&self) -> i64 {
        unimplemented!()
    }

    /// _Inherited from Spiderling_
    ///
    /// The Web this Spiderling is using to move. None if it is not moving.
    pub fn moving_on_web(&self) -> Option<Web> {
        unimplemented!()
    }

    /// _Inherited from Spiderling_
    ///
    /// The Nest this Spiderling is moving to. None if it is not moving.
    pub fn moving_to_nest(&self) -> Option<Nest> {
        unimplemented!()
    }

    /// _Inherited from Spider_
    ///
    /// The Player that owns this Spider, and can command it.
    pub fn owner(&self) -> Player {
        unimplemented!()
    }

    /// _Inherited from Spider_
    ///
    /// The Nest that this Spider is currently on. None when moving on a Web.
    pub fn nest(&self) -> Option<Nest> {
        unimplemented!()
    }

    /// _Inherited from Spider_
    ///
    /// If this Spider is dead and has been removed from the game.
    pub fn is_dead(&self) -> bool {
        unimplemented!()
    }

    /// _Inherited from GameObject_
    ///
    /// A unique id for each instance of a GameObject or a sub class. Used for client and server
    /// communication. Should never change value after being set.
    pub fn id(&self) -> Str {
        unimplemented!()
    }

    /// _Inherited from GameObject_
    ///
    /// String representing the top level Class that this game object is an instance of. Used for
    /// reflection to create new instances on clients, but exposed for convenience should AIs want
    /// this data.
    pub fn game_object_name(&self) -> Str {
        unimplemented!()
    }

    /// _Inherited from GameObject_
    ///
    /// Any strings logged will be stored here. Intended for debugging.
    pub fn logs(&self) -> List<Str> {
        unimplemented!()
    }

    /// Cuts a web, destroying it, and any Spiderlings on it.
    ///
    /// # Arguments
    ///
    /// - _web_ - The web you want to Cut. Must be connected to the Nest this Cutter is currently
    /// on.
    ///
    /// # Returns
    ///
    /// True if the cut was successfully started, false otherwise.
    pub fn cut(
        &self,
        _web: &Web,
    )
        -> bool
    {
        unimplemented!()
    }

    /// _Inherited from Spiderling_
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
    pub fn move_to(
        &self,
        _web: &Web,
    )
        -> bool
    {
        unimplemented!()
    }

    /// _Inherited from Spiderling_
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
        -> bool
    {
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
        _message: &str,
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
