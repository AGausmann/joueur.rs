#![allow(dead_code, unused_imports)]

use super::*;
use crate::util::*;

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

    /// This is called whenever your checker gets captured (during an opponent's turn).
    ///
    /// # Arguments
    ///
    /// - _checker_ - The checker that was captured.
    pub fn got_captured(
        &self,
        _checker: &Checker,
    )
    {
        unimplemented!()
    }
}
