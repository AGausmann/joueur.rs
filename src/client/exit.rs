//! Exit codes used by the game client to communicate exit reasons to the arena.
//!
//! Source: https://github.com/siggame/Cadre/blob/master/Docs/error-codes.md

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
