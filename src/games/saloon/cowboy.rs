#![allow(dead_code, unused_imports)]

use super::*;
use crate::util::*;

/// A person on the map that can move around and interact within the saloon.
pub struct Cowboy {
}

impl Cowboy {

    /// How much health this Cowboy currently has.
    pub fn health(&self) -> isize {
        unimplemented!()
    }

    /// The Player that owns and can control this Cowboy.
    pub fn owner(&self) -> Player {
        unimplemented!()
    }

    /// If this Cowboy is dead and has been removed from the game.
    pub fn is_dead(&self) -> bool {
        unimplemented!()
    }

    /// The job that this Cowboy does, and dictates how they fight and interact within the Saloon.
    pub fn job(&self) -> String {
        unimplemented!()
    }

    /// If the Cowboy can be moved this turn via its owner.
    pub fn can_move(&self) -> bool {
        unimplemented!()
    }

    /// The Tile that this Cowboy is located on.
    pub fn tile(&self) -> Option<Tile> {
        unimplemented!()
    }

    /// How much focus this Cowboy has. Different Jobs do different things with their Cowboy's
    /// focus.
    pub fn focus(&self) -> isize {
        unimplemented!()
    }

    /// If this Cowboy is drunk, and will automatically walk.
    pub fn is_drunk(&self) -> bool {
        unimplemented!()
    }

    /// The direction this Cowboy is moving while drunk. Will be 'North', 'East', 'South', or
    /// 'West' when drunk; or '' (empty string) when not drunk.
    pub fn drunk_direction(&self) -> String {
        unimplemented!()
    }

    /// How many times this unit has been drunk before taking their siesta and reseting this to 0.
    pub fn tolerance(&self) -> isize {
        unimplemented!()
    }

    /// How many turns this unit has remaining before it is no longer busy and can `act()` or
    /// `play()` again.
    pub fn turns_busy(&self) -> isize {
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

    /// Moves this Cowboy from its current Tile to an adjacent Tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile you want to move this Cowboy to.
    ///
    /// # Returns
    ///
    /// True if the move worked, false otherwise.
    pub fn move_to(
        &self,
        _tile: &Tile,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Sits down and plays a piano.
    ///
    /// # Arguments
    ///
    /// - _piano_ - The Furnishing that is a piano you want to play.
    ///
    /// # Returns
    ///
    /// True if the play worked, false otherwise.
    pub fn play(
        &self,
        _piano: &Furnishing,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Does their job's action on a Tile.
    ///
    /// # Arguments
    ///
    /// - _tile_ - The Tile you want this Cowboy to act on.
    ///
    /// - _drunk_direction_ - The direction the bottle will cause drunk cowboys to be in, can be
    /// 'North', 'East', 'South', or 'West'.
    ///
    /// # Returns
    ///
    /// True if the act worked, false otherwise.
    pub fn act(
        &self,
        _tile: &Tile,
        _drunk_direction: &String,
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
