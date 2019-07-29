#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;

/// The AI competitors will add logic to this AI so its functions return valid and smart values.
pub struct AI {
}

impl AI {

    /// This is called every time it is this AI.player's turn.
    ///
    /// # Returns
    ///
    /// Represents if you want to end your turn. True means end your turn, False means to keep your
    /// turn going and re-call this function.
    pub fn run_turn(
        &self,
    )
        -> bool
    {
        unimplemented!()
    }
}
