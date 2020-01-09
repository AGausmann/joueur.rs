//! The game server protocol, implemented based on Cadre's [Client-Server
//! IO](https://github.com/siggame/Cadre/blob/master/client-server-io.md) documentation.

use std::collections::HashMap;
use std::io::{self, BufRead, Write};
use std::marker::PhantomData;
use std::time::UNIX_EPOCH;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;
use serde_derive::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Id {
    id: String,
}

/// The list of events that clients are allowed to send.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", content = "data", rename_all = "camelCase")]
pub enum ClientEvent {
    /// Asks the game server what the actual game name (id) is for some game alias is. The returned
    /// "named" event's `data` will be a string of the actual name. If the alias is unknown an
    /// "error" event will be sent and the client will be force disconnected from the server.
    /// # Example
    ///
    /// ```json
    /// {
    ///     "event": "alias",
    ///     "data": "sP2017-ChEsS"
    /// }
    /// ```
    Alias(String),

    /// The Client tells the server it wants to play some game in some session. Once this occurs
    /// the server should soon after reply that it has been moved to some game lobby waiting for
    /// that game to starting playing.
    ///
    /// # Example
    ///
    /// ```json
    /// {
    ///     "event": "play",
    ///    "data": {
    ///         "gameName": "Chess",
    ///         "requestedSession": "BestSession",
    ///         "clientType": "C++",
    ///         "playerName": "John Smith",
    ///         "playerIndex": 0,
    ///         "password": "",
    ///         "gameSettings": "startTime=300000&something=somethingElse"
    ///     }
    /// }
    /// ```
    #[serde(rename_all = "camelCase")]
    Play {
        /// The name (id) of the game to play. Use the "alias" event to get the actual id before
        /// this.
        game_name: String,

        /// An identifier for the game session you want to play in. If omitted it means you want to
        /// play in the first available session of that game. Send a string if you want to play in
        /// a private session, like "MyPrivateGameSession". _Note_: this is _requested_ the actual
        /// session you are put in may differ if the given string is "new", "*", or taken by an in
        /// progress game.
        requested_session: String,

        /// The programming language this client is. This should probably be hard coded and not
        /// able to be changed by the client.
        client_type: String,

        /// The name the of player the client is working on behalf of.
        player_name: String,

        /// The preferred player index this player wants to play as. By default if this is omitted
        /// the first player to connect is the first player in the game, however you can override
        /// that by sending a number, so if the second player to connect sends `0`, then they will
        /// be the first player in the game (0 is first, not 1, think arrays).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        player_index: Option<isize>,

        /// If the game server has authentication enabled, this is the password to be allowed to
        /// play on said server.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        password: Option<String>,

        /// Settings for the game. This varies based on each game and there is no way for a client
        /// to know which game settings are valid. Instead send a [Query
        /// string](https://en.wikipedia.org/wiki/Query_string) formatted string of the settings.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        game_settings: Option<String>,

        /// If set to `true`, then this client is treated as a spectator and will not play, but
        /// will still be sent information about the game as it progresses. Any other value will be
        /// treated as false (such as omitting the key).
        #[serde(default, skip_serializing_if = "Option::is_none")]
        spectating: Option<bool>,
    },

    /// Sent from a client to the server as the result of an "order" event. This is basically the
    /// return event from said order.
    ///
    ///
    /// # Example
    ///
    /// ```json
    /// {
    ///     "event": "finished",
    ///     "data": {
    ///         "orderIndex": 0,
    ///         "returned": true,
    ///     }
    /// }
    /// ```
    #[serde(rename_all = "camelCase")]
    Finished {
        /// The index of the order the client finished running
        order_index: isize,

        /// The value returned from that order
        returned: Value,
    },

    /// Sent from the client to the game server as a request to run some game logic server side.
    /// The server should eventually send back a "ran" event, but before that expect other events
    /// such as a "delta" for how this ran changed the game state.
    ///
    /// # Example
    ///
    /// ```json
    /// {
    ///     "event": "run",
    ///     "data": {
    ///         "caller": {"id": "20"},
    ///         "functionName": "move",
    ///         "args": {
    ///             "file": "d",
    ///             "promotionType": "",
    ///             "rank": 1
    ///         }
    ///     }
    /// }
    /// ```
    #[serde(rename_all = "camelCase")]
    Run {
        /// The game object's reference (object with just its ID) to the game object calling this
        /// run command
        caller: Id,

        /// The name of the function to run on the caller
        function_name: String,

        /// The arguments for the function being ran. This will be a key/value dictionary where
        /// each key is the string of the argument to the function by name, and the value is the
        /// value for that argument (type varies by function obviously)
        args: HashMap<String, Value>,
    },
}

/// The list of events that servers are allowed to send.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "event", content = "data", rename_all = "camelCase")]
pub enum ServerEvent {
    /// Sent back from a client event "alias" to name the sent alias. Sends back `data` that is a
    /// string of the actual name (id).
    ///
    /// # Example
    ///
    /// Server --> Client:
    ///
    /// ```json
    /// {
    ///     "event": "named",
    ///     "data": "Chess"
    /// }
    /// ```
    Named(String),

    /// This should be sent after a client sends a "play" event, notifying them they are in a lobby
    /// waiting for the game to start. After this event clients should wait for a "start" even to
    /// be sent from the server when the game starts.
    ///
    /// After this point the game client should have enough information to initialize classes for
    /// its AI and Game instances.
    ///
    /// # Example
    ///
    /// Server --> Client:
    ///
    /// ```json
    /// {
    ///     "event": "lobbied",
    ///     "data": {
    ///         "gameName": "Chess",
    ///         "gameSession": "BestSession",
    ///         "constants": {
    ///             "DELTA_REMOVED":"&RM",
    ///             "DELTA_LIST_LENGTH":"&LEN"
    ///         }
    ///     }
    /// }
    /// ```
    #[serde(rename_all = "camelCase")]
    Lobbied {
        /// The name of the game that is starting. Should be the same as
        /// the name you send via "play"
        game_name: String,

        /// The game session you actually ended up in. May differ from the sent `requestedSession`.
        game_session: String,

        /// A string key to string value dictionary. Clients **must** save this somewhere, as the
        /// constants sent back from this will be used from the server to encode future messages
        /// such as delta changed events.
        constants: HashMap<String, String>,
    },

    /// This event will occur _any time a change in state occurs on the game server_ (except
    /// players' timers ticking, as time is always changing). The delta's exact makeup cannot be
    /// predicted, as it depends wholly on the game being played's structure, and what changed.
    /// Delta merging is a crucial concept to understand to merge game states, that is outside the
    /// scope of this document.
    ///
    /// The `data` property will always be an object that represents the game's current state, but
    /// that object will only contain changed keys/values. Clients should be able to delta merge
    /// that object on top of the current Game.
    ///
    /// __Note__: This event will occur while game play is occurring, but it will also be sent
    /// _once_ before. Right before the "start" event is sent, this initial "delta" event can be
    /// seen as a change in state of the game from un-started to the initial game state.
    Delta(HashMap<String, Value>),

    /// This will eventually be sent from the server to the client when the game starts. This event
    /// is dependent on other client(s) connecting to start the game. When this is sent the client
    /// and game should shift to Game mode, where events are exchanged quickly to do gameplay. All
    /// prior events not marked Client/Server Game Event, are no longer valid and will never be
    /// seen again.
    ///
    /// The sent data is optional. If it is sent it contains game information pertinent to only
    /// your client.
    ///
    /// # Example
    ///
    /// ```json
    /// {
    ///     "event":"start",
    ///     "data":{
    ///         "playerID":"0"
    ///     }
    /// }
    /// ```
    #[serde(rename_all = "camelCase")]
    Start {
        /// The player index of your player in the `Game.players` array. Synonymous with
        /// `PlayerIndex`.
        #[serde(rename = "playerID")]
        player_id: String,
    },

    /// Sent from the game server instructing them to fulfill some order. Probably "runTurn". You
    /// can think of this as the server telling the client to run a function and send back the
    /// result. While the order is happening clients may send certain "run" events depending on the
    /// game.
    ///
    /// # Example
    ///
    /// ```json
    /// {
    ///     "event":"order",
    ///     "data": {
    ///         "name":"runTurn",
    ///         "index":0,
    ///         "args":[]
    ///     }
    /// }
    /// ```
    #[serde(rename_all = "camelCase")]
    Order {
        /// The name of the order to execute (probably a function name in camelCase).
        name: String,

        /// An identifier for the order. When the client finishes executing this order this index
        /// should be sent back too (just in-case of duplicate identical orders).
        index: usize,

        /// An array of arguments to be piped to the order's named function, in positional order.
        args: Vec<Value>,
    },

    /// The response to a client's run event. This occurs once it has been, well, ran. The `data`
    /// send from this is optional. If the requested 'run' would have a return value, that is sent
    /// as the `data` value for this command.
    ///
    /// # Example
    ///
    /// ```json
    /// {
    ///     "event": "ran",
    ///     "data": {"id": "34"}
    /// }
    /// ```
    Ran(Value),

    /// An event that should not occur during normal operations. This is often sent if a client
    /// sends a "run" event with invalid properties, such as trying to run a function on a
    /// GameObject they do not own. These should result in printing a yellow message telling the
    /// coder why the run is invalid. The message will be in the `data` part of the event as a
    /// stand alone key `message`.
    ///
    /// _Note_: a "ran" even will still be sent back after the "invalid" event. This is an
    /// _additional_ event.
    ///
    /// # Example
    ///
    /// ```json
    /// {
    ///     "event":"invalid",
    ///     "data": {
    ///         "message":"Tried to Move Piece White Pawn #20 at c2 to d1."
    ///     }
    /// }
    /// ```
    #[serde(rename_all = "camelCase")]
    Invalid { message: String },

    /// This occurs when the game is over. Once this event is sent no further events can be sent
    /// from either party. In addition the game server will disconnect automatically.
    ///
    /// With the game being over, the last state received should have players set if they won or
    /// lost. That data is not directly sent in this event however.
    ///
    /// # `data` options
    ///
    /// The entire `data` object is optional, but game servers may elect to send additional
    /// meta-data about the end of the game, such as gamelog information. Because the game server's
    /// hostname may vary from client to client, any strings with the term `__HOSTNAME__` in them
    /// should have that substring replaced by the hostname they used to connect to the game
    /// server. This is expected client side behavior.
    ///
    /// # Example
    ///
    /// ```json
    /// {
    ///     "event": "over",
    ///     "data": {
    ///         "gamelogURL": "http://__HOSTNAME__:3080/gamelog/2017.08.18.13.26.44.351-Chess-1",
    ///         "visualizerURL": "http://vis.siggame.io/?log=http%3A%2F%2F__HOSTNAME__%3A3080%2Fgamelog%2F2017.08.18.13.26.44.351-Chess-1",
    ///         "message": "---\nYour gamelog is viewable at:\nhttp://vis.siggame.io/?log=http%3A%2F%2F__HOSTNAME__%3A3080%2Fgamelog%2F2017.08.18.13.26.44.351-Chess-1\n---"
    ///     }
    /// }
    /// ```
    #[serde(rename_all = "camelCase")]
    Over {
        /// The url to the gamelog, in case it needs to be downloaded.
        #[serde(
            rename = "gamelogURL",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        gamelog_url: Option<String>,

        /// The url to the visualizer which will download the gamelog and play it back.
        #[serde(
            rename = "visualizerURL",
            default,
            skip_serializing_if = "Option::is_none"
        )]
        visualizer_url: Option<String>,

        /// A message to display to the coder running the client. It will probably tell them that
        /// the game is over and they can watch their gamelog via a link embed in the message.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        message: Option<String>,
    },

    /// This event should never happen, but may do so during development. This means a fatal error
    /// was thrown on the server for some reason, and it could not recover. The game server
    /// disconnects after this message, but at least the game clients are given some reason why
    /// they were disconnected unexpectedly.
    ///
    /// # Example
    ///
    /// ```json
    /// {
    ///     "event": "fatal",
    ///     "data": {
    ///         "message": "An un-handled fatal error has occurred on the server."
    ///     }
    /// }
    /// ```
    #[serde(rename_all = "camelCase")]
    Fatal {
        /// The error message about what fatal error occurred.
        #[serde(default, skip_serializing_if = "Option::is_none")]
        message: Option<String>,
    },
}

/// A full message, containing an event and a Unix epoch timestamp.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Message<T> {
    /// The event packed with this message.
    #[serde(flatten)]
    pub event: T,

    /// The duration, in seconds, from Unix epoch to the time that this message was sent.
    pub sent_time: u64,
}

const EOT: u8 = 0x04;

/// Reads and parses events from a stream that conforms to the Cadre game server protocol.
#[derive(Debug)]
pub struct EventStream<R, T> {
    split: io::Split<R>,
    _t: PhantomData<T>,
}

impl<R, T> EventStream<R, T>
where
    R: BufRead,
    T: DeserializeOwned,
{
    /// Wraps the given reader.
    pub fn new(buf_read: R) -> EventStream<R, T> {
        EventStream {
            split: buf_read.split(EOT),
            _t: PhantomData,
        }
    }

    /// Receives the next event if one is available.
    pub fn recv(&mut self) -> Option<io::Result<T>> {
        self.split.next().map(|result| {
            result.and_then(|bytes| serde_json::from_slice(&bytes).map_err(Into::into))
        })
    }
}

impl<R, T> Iterator for EventStream<R, T>
where
    R: BufRead,
    T: DeserializeOwned,
{
    type Item = io::Result<T>;

    fn next(&mut self) -> Option<Self::Item> {
        self.recv()
    }
}

/// An event serializer/writer that conforms to the Cadre game server protocol.
#[derive(Debug)]
pub struct EventSink<W, T> {
    write: W,
    _t: PhantomData<T>,
}

impl<W, T> EventSink<W, T>
where
    W: Write,
{
    /// Wraps the given writer.
    pub fn new(write: W) -> EventSink<W, T> {
        EventSink {
            write,
            _t: PhantomData,
        }
    }

    /// Serializes and sends an event, adding a valid timestamp to the message.
    pub fn send(&mut self, event: T) -> io::Result<()>
    where
        T: Serialize,
    {
        let message = Message {
            event,
            sent_time: UNIX_EPOCH.elapsed().expect("invalid system time").as_secs(),
        };
        serde_json::to_writer(&mut self.write, &message)?;
        self.write.write_all(&[EOT])?;
        self.write.flush()?;
        Ok(())
    }
}
