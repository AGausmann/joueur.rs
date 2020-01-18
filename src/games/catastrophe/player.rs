#![allow(dead_code, unused_imports)]

use std::marker::PhantomData;
use std::sync::{Arc, Mutex, MutexGuard, Weak};

use super::*;
use crate::types::*;
use crate::error::Error;

/// A player in this game. Every AI controls one player.
#[derive(Debug, Clone)]
pub struct Player {
    context: Weak<Mutex<inner::Context>>,
    inner: Arc<Mutex<inner::GameObject>>,
}

impl Player {
    fn with_context<F, R>(&self, f: F) -> R
    where
        F: FnOnce(&mut inner::Context) -> R,
    {
        let context = self.context.upgrade().expect("context dropped before end of game");
        let mut handle = context.lock().unwrap();
        f(&mut handle)
    }

    /// The name of the player.
    pub fn name(&self) -> Str {
        self.inner.lock().unwrap().as_player()
            .name.clone()
    }

    /// What type of client this is, e.g. 'Python', 'JavaScript', or some other language. For
    /// potential data mining purposes.
    pub fn client_type(&self) -> Str {
        self.inner.lock().unwrap().as_player()
            .client_type.clone()
    }

    /// If the player won the game or not.
    pub fn won(&self) -> bool {
        self.inner.lock().unwrap().as_player()
            .won.clone()
    }

    /// If the player lost the game or not.
    pub fn lost(&self) -> bool {
        self.inner.lock().unwrap().as_player()
            .lost.clone()
    }

    /// The reason why the player won the game.
    pub fn reason_won(&self) -> Str {
        self.inner.lock().unwrap().as_player()
            .reason_won.clone()
    }

    /// The reason why the player lost the game.
    pub fn reason_lost(&self) -> Str {
        self.inner.lock().unwrap().as_player()
            .reason_lost.clone()
    }

    /// The amount of time (in ns) remaining for this AI to send commands.
    pub fn time_remaining(&self) -> f64 {
        self.inner.lock().unwrap().as_player()
            .time_remaining.clone()
    }

    /// This player's opponent in the game.
    pub fn opponent(&self) -> Player {
        self.inner.lock().unwrap().as_player()
            .opponent.clone()
    }

    /// Every Unit owned by this Player.
    pub fn units(&self) -> List<Unit> {
        self.inner.lock().unwrap().as_player()
            .units.clone()
    }

    /// The overlord cat Unit owned by this Player.
    pub fn cat(&self) -> Unit {
        self.inner.lock().unwrap().as_player()
            .cat.clone()
    }

    /// The total upkeep of every Unit owned by this Player. If there isn't enough food for every
    /// Unit, all Units become starved and do not consume food.
    pub fn upkeep(&self) -> i64 {
        self.inner.lock().unwrap().as_player()
            .upkeep.clone()
    }

    /// Every Structure owned by this Player.
    pub fn structures(&self) -> List<Structure> {
        self.inner.lock().unwrap().as_player()
            .structures.clone()
    }

    /// The amount of food owned by this player.
    pub fn food(&self) -> i64 {
        self.inner.lock().unwrap().as_player()
            .food.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// A unique id for each instance of a GameObject or a sub class. Used for client and server
    /// communication. Should never change value after being set.
    pub fn id(&self) -> Str {
        self.inner.lock().unwrap().as_game_object()
            .id.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// String representing the top level Class that this game object is an instance of. Used for
    /// reflection to create new instances on clients, but exposed for convenience should AIs want
    /// this data.
    pub fn game_object_name(&self) -> Str {
        self.inner.lock().unwrap().as_game_object()
            .game_object_name.clone()
    }

    /// _Inherited from [`GameObject`]_
    ///
    /// Any strings logged will be stored here. Intended for debugging.
    pub fn logs(&self) -> List<Str> {
        self.inner.lock().unwrap().as_game_object()
            .logs.clone()
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
        message: &str,
    )
        -> Result<(), Error>
    {
        struct Args<'a> {
            message: &'a str,
            _a: PhantomData< &'a () >,
        }
        let args = Args {
            message,
            _a: PhantomData,
        };
        self.with_context(|cx| cx.run(&self.id(), "log", args))
    }

    pub fn try_cast<T: Object>(&self) -> Option<T> {
        T::from_game_object(&self.inner, &self.context)
    }

    pub fn cast<T: Object>(&self) -> T {
        self.try_cast().unwrap()
    }
}

impl inner::ObjectInner for Player {
    fn from_game_object(game_obj: &Arc<Mutex<inner::GameObject>>, context: &Weak<Mutex<inner::Context>>) -> Option<Self> {
        let handle = game_obj.lock().unwrap();
        if handle.try_as_player().is_some() {
            Some(Player {
                inner: Arc::clone(&game_obj),
                context: context.clone(),
            })
        } else {
            None
        }
    }
}
impl Object for Player {}
