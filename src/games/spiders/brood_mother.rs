#![allow(dead_code, unused_imports)]

use super::*;
use crate::util::*;

/// The Spider Queen. She alone can spawn Spiderlings for each Player, and if she dies the owner
/// loses.
pub struct BroodMother {
}

impl BroodMother {

    /// How much health this BroodMother has left. When it reaches 0, she dies and her owner loses.
    pub fn health(&self) -> isize {
        unimplemented!()
    }

    /// How many eggs the BroodMother has to spawn Spiderlings this turn.
    pub fn eggs(&self) -> isize {
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
    /// The Nest that this Spider is currently on. Null when moving on a Web.
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

    /// Consumes a Spiderling of the same owner to regain some eggs to spawn more Spiderlings.
    ///
    /// # Arguments
    ///
    /// - _spiderling_ - The Spiderling to consume. It must be on the same Nest as this
    /// BroodMother.
    ///
    /// # Returns
    ///
    /// True if the Spiderling was consumed. False otherwise.
    pub fn consume(
        &self,
        _spiderling: &Spiderling,
    )
        -> bool
    {
        unimplemented!()
    }

    /// Spawns a new Spiderling on the same Nest as this BroodMother, consuming an egg.
    ///
    /// # Arguments
    ///
    /// - _spiderling_type_ - The string name of the Spiderling class you want to Spawn. Must be
    /// 'Spitter', 'Weaver', or 'Cutter'.
    ///
    /// # Returns
    ///
    /// The newly spwaned Spiderling if successful. Null otherwise.
    pub fn spawn(
        &self,
        _spiderling_type: &String,
    )
        -> Option<Spiderling>
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
