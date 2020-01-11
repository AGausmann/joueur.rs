//! Exit codes used by the game client to communicate exit reasons to the arena.
//!
//! Source: https://github.com/siggame/Cadre/blob/master/Docs/error-codes.md

use std::fmt;

pub const NONE: i32 = 0;
pub const INVALID_ARGS: i32 = 20;
pub const COULD_NOT_CONNECT: i32 = 21;
pub const DISCONNECTED_UNEXPECTEDLY: i32 = 22;
pub const CANNOT_READ_SOCKET: i32 = 23;
pub const DELTA_MERGE_FAILURE: i32 = 24;
pub const REFLECTION_FAILED: i32 = 25;
pub const UNKNOWN_EVENT_FROM_SERVER: i32 = 26;
pub const SERVER_TIMEOUT: i32 = 27;
pub const FATAL_EVENT: i32 = 28;
pub const GAME_NOT_FOUND: i32 = 29;
pub const MALFORMED_JSON: i32 = 30;
pub const UNAUTHENTICATED: i32 = 31;
pub const AI_ERRORED: i32 = 42;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Exit {
    None,
    InvalidArgs,
    CouldNotConnect,
    DisconnectedUnexpectedly,
    CannotReadSocket,
    DeltaMergeFailure,
    ReflectionFailed,
    UnknownEventFromServer,
    ServerTimeout,
    FatalEvent,
    GameNotFound,
    MalformedJson,
    Unauthenticated,
    AiErrored,
}

impl Exit {
    pub fn code(&self) -> i32 {
        match self {
            Exit::None => NONE,
            Exit::InvalidArgs => INVALID_ARGS,
            Exit::CouldNotConnect => COULD_NOT_CONNECT,
            Exit::DisconnectedUnexpectedly => DISCONNECTED_UNEXPECTEDLY,
            Exit::CannotReadSocket => CANNOT_READ_SOCKET,
            Exit::DeltaMergeFailure => DELTA_MERGE_FAILURE,
            Exit::ReflectionFailed => REFLECTION_FAILED,
            Exit::UnknownEventFromServer => UNKNOWN_EVENT_FROM_SERVER,
            Exit::ServerTimeout => SERVER_TIMEOUT,
            Exit::FatalEvent => FATAL_EVENT,
            Exit::GameNotFound => GAME_NOT_FOUND,
            Exit::MalformedJson => MALFORMED_JSON,
            Exit::Unauthenticated => UNAUTHENTICATED,
            Exit::AiErrored => AI_ERRORED,
        }
    }
}

impl fmt::Display for Exit {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Exit::None => write!(f, "no error"),
            Exit::InvalidArgs => write!(f, "invalid command line arguments"),
            Exit::CouldNotConnect => write!(f, "could not connect to server"),
            Exit::DisconnectedUnexpectedly => write!(f, "disconnected unexpectedly"),
            Exit::CannotReadSocket => write!(f, "cannot read socket"),
            Exit::DeltaMergeFailure => write!(f, "delta merge failure"),
            Exit::ReflectionFailed => write!(f, "reflection failed"),
            Exit::UnknownEventFromServer => write!(f, "unknown or unexpected event from server"),
            Exit::ServerTimeout => write!(f, "connection to server timed out"),
            Exit::FatalEvent => write!(f, "received fatal event from server"),
            Exit::GameNotFound => write!(f, "game not found"),
            Exit::MalformedJson => write!(f, "received malformed JSON from server"),
            Exit::Unauthenticated => write!(f, "authentication failed"),
            Exit::AiErrored => write!(f, "ai error"),
        }
    }
}
