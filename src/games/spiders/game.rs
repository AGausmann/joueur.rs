#![allow(dead_code, unused_imports)]

use super::*;
use crate::util::*;

/// There's an infestation of enemy spiders challenging your queen broodmother spider! Protect her
/// and attack the other broodmother in this turn based, node based, game.
pub struct Game {
}

impl Game {

    /// A mapping of every game object's ID to the actual game object. Primarily used by the server
    /// and client to easily refer to the game objects via ID.
    pub fn game_objects(&self) -> Map<String, GameObject> {
        unimplemented!()
    }

    /// List of all the players in the game.
    pub fn players(&self) -> List<Player> {
        unimplemented!()
    }

    /// A unique identifier for the game instance that is being played.
    pub fn session(&self) -> String {
        unimplemented!()
    }

    /// The player whose turn it is currently. That player can send commands. Other players cannot.
    pub fn current_player(&self) -> Player {
        unimplemented!()
    }

    /// The current turn number, starting at 0 for the first player's turn.
    pub fn current_turn(&self) -> isize {
        unimplemented!()
    }

    /// The maximum number of turns before the game will automatically end.
    pub fn max_turns(&self) -> isize {
        unimplemented!()
    }

    /// The amount of time (in nano-seconds) added after each player performs a turn.
    pub fn time_added_per_turn(&self) -> isize {
        unimplemented!()
    }

    /// Every Nest in the game.
    pub fn nests(&self) -> List<Nest> {
        unimplemented!()
    }

    /// Every Web in the game.
    pub fn webs(&self) -> List<Web> {
        unimplemented!()
    }

    /// The speed at which Spiderlings move on Webs.
    pub fn movement_speed(&self) -> isize {
        unimplemented!()
    }

    /// The speed at which Weavers work to do strengthens and weakens on Webs.
    pub fn weave_speed(&self) -> isize {
        unimplemented!()
    }

    /// The speed at which Cutters work to do cut Webs.
    pub fn cut_speed(&self) -> isize {
        unimplemented!()
    }

    /// The speed at which Spitters work to spit new Webs.
    pub fn spit_speed(&self) -> isize {
        unimplemented!()
    }

    /// How much web strength is added or removed from Webs when they are weaved.
    pub fn weave_power(&self) -> isize {
        unimplemented!()
    }

    /// The starting strength for Webs.
    pub fn initial_web_strength(&self) -> isize {
        unimplemented!()
    }

    /// The maximum strength a web can be strengthened to.
    pub fn max_web_strength(&self) -> isize {
        unimplemented!()
    }

    /// Constant used to calculate how many eggs BroodMothers get on their owner's turns.
    pub fn eggs_scalar(&self) -> f64 {
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
