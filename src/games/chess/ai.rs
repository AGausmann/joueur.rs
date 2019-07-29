#![allow(dead_code, unused_imports)]

use super::*;
use crate::types::*;

/// The AI competitors will add logic to this AI so its functions return valid and smart values.
pub struct AI {
}

impl AI {

    /// This is called every time it is this AI.player's turn to make a move.
    ///
    /// # Returns
    ///
    /// A string in Standard Algebriac Notation (SAN) for the move you want to make. If the move is
    /// invalid or not properly formatted you will lose the game.
    pub fn make_move(
        &self,
    )
        -> Str
    {
        unimplemented!()
    }
}
