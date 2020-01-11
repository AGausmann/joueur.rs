use std::env;
use std::error::Error as StdError;
use std::fmt;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use std::process::exit;

use joueur::client::exit::Exit;
use joueur::client::proto::{self, ClientEvent, EventSink, EventStream, ServerEvent};
use serde_json::error::Category;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Args {
    /// the name of the game you want to play on the server
    game: String,

    /// the hostname or the server you want to connect to e.g. localhost:3000
    #[structopt(short = "s", long = "server", default_value = "localhost")]
    server: String,

    /// the port to connect to on the server. Can be defined on the server arg via server:port
    #[structopt(short = "p", long = "port", default_value = "3000")]
    port: u16,

    /// the name you want to use as your AI's player name. This over-rides the name you set in your code
    #[structopt(short = "n", long = "name")]
    name: Option<String>,

    /// the player number you want to be, with 0 being the first player
    #[structopt(short = "i", long = "index")]
    index: Option<isize>,

    /// the password required for authentication on official servers
    #[structopt(short = "w", long = "password")]
    password: Option<String>,

    /// the requested game session you want to play on the server
    #[structopt(short = "r", long = "session", default_value = "*")]
    session: String,

    /// Any settings for the game server to force. Must be query string formatted (key=value&otherKey=otherValue)
    #[structopt(long = "gameSettings")]
    game_settings: Option<String>,

    /// Any settings for the AI. Delimit pairs by an ampersand (key=value&otherKey=otherValue)
    #[structopt(long = "aiSettings")]
    ai_settings: Option<String>,

    /// (debugging) print IO through the TCP socket to the terminal
    #[structopt(long = "printIO")]
    print_io: bool,
}

fn main_sub() -> Result<(), Error> {
    let args = Args::from_iter_safe(env::args_os()).map_err(|e| (Exit::InvalidArgs, e))?;

    let mut server_parts = args.server.split(":");
    let server_name = server_parts.next().unwrap();
    let server_port = match server_parts.next() {
        Some(port_str) => port_str.parse().map_err(|e| (Exit::InvalidArgs, e))?,
        None => args.port,
    };

    let socket =
        TcpStream::connect((server_name, server_port)).map_err(|e| (Exit::CouldNotConnect, e))?;
    let read_half = socket.try_clone().map_err(|e| (Exit::CouldNotConnect, e))?;
    let write_half = socket;

    let mut stream: EventStream<_, ServerEvent> = EventStream::new(BufReader::new(PrintRead {
        read: read_half,
        print_io: args.print_io,
    }));
    let mut sink: EventSink<_, ClientEvent> = EventSink::new(BufWriter::new(PrintWrite {
        write: write_half,
        print_io: args.print_io,
    }));

    sink.send(ClientEvent::Alias(args.game))?;
    let game_name = match stream.recv().ok_or(Exit::DisconnectedUnexpectedly)?? {
        ServerEvent::Named(name) => name,
        event => return Err(unexpected_event(event)),
    };

    sink.send(ClientEvent::Play {
        game_name,
        requested_session: args.session,
        client_type: "Rust".into(),
        player_name: args.name.expect("todo: get name from ai"),
        player_index: args.index,
        password: args.password,
        game_settings: args.game_settings,
        spectating: None,
    })?;
    let (game_name, game_session, constants) =
        match stream.recv().ok_or(Exit::DisconnectedUnexpectedly)?? {
            ServerEvent::Lobbied {
                game_name,
                game_session,
                constants,
            } => (game_name, game_session, constants),
            event => return Err(unexpected_event(event)),
        };

    let initial_delta = match stream.recv().ok_or(Exit::DisconnectedUnexpectedly)?? {
        ServerEvent::Delta(delta) => delta,
        event => return Err(unexpected_event(event)),
    };
    let player_id = match stream.recv().ok_or(Exit::DisconnectedUnexpectedly)?? {
        ServerEvent::Start { player_id } => player_id,
        event => return Err(unexpected_event(event)),
    };

    //TODO hand off to game runtime

    Ok(())
}

fn unexpected_event(event: ServerEvent) -> Error {
    match event {
        ServerEvent::Fatal { message } => {
            let message = message.unwrap_or_else(|| "unknown cause".into());
            (Exit::FatalEvent, message).into()
        }
        event => (Exit::UnknownEventFromServer, format!("{:?}", event)).into(),
    }
}

#[derive(Debug)]
struct Error {
    source: Option<Box<dyn StdError>>,
    exit: Option<Exit>,
}

impl Error {
    fn exit_code(&self) -> i32 {
        match &self.exit {
            Some(exit) => exit.code(),
            None => 1,
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.source, &self.exit) {
            (Some(source), Some(exit)) => write!(f, "{}: {}", exit, source),
            (Some(source), None) => write!(f, "{}", source),
            (None, Some(exit)) => write!(f, "{}", exit),
            (None, None) => write!(f, "unknown error"),
        }
    }
}

impl<E> From<(Exit, E)> for Error
where
    E: Into<Box<dyn StdError>>,
{
    fn from((exit, err): (Exit, E)) -> Error {
        Error {
            source: Some(err.into()),
            exit: Some(exit),
        }
    }
}

impl From<Exit> for Error {
    fn from(exit: Exit) -> Error {
        Error {
            source: None,
            exit: Some(exit),
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Error {
        use io::ErrorKind::*;
        match err.kind() {
            NotConnected => Exit::CouldNotConnect.into(),
            ConnectionAborted | ConnectionReset | BrokenPipe => {
                (Exit::DisconnectedUnexpectedly, err).into()
            }
            TimedOut => Exit::ServerTimeout.into(),
            _ => (Exit::CannotReadSocket, err).into(),
        }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Error {
        match err.classify() {
            Category::Io | Category::Eof => io::Error::from(err).into(),
            Category::Syntax => (Exit::MalformedJson, err).into(),
            Category::Data => (Exit::UnknownEventFromServer, err).into(),
        }
    }
}

impl From<proto::Error> for Error {
    fn from(err: proto::Error) -> Error {
        match err {
            proto::Error::Io(err) => err.into(),
            proto::Error::Json(err) => err.into(),
        }
    }
}

struct PrintRead<R> {
    read: R,
    print_io: bool,
}

impl<R> Read for PrintRead<R>
where
    R: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        let n = self.read.read(buf)?;
        if self.print_io && n > 0 {
            eprintln!("FROM SERVER <-- {}", String::from_utf8_lossy(&buf[..n]));
        }
        Ok(n)
    }
}

struct PrintWrite<W> {
    write: W,
    print_io: bool,
}

impl<W> Write for PrintWrite<W>
where
    W: Write,
{
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        let n = self.write.write(buf)?;
        if self.print_io && n > 0 {
            eprintln!("TO SERVER --> {}", String::from_utf8_lossy(&buf[..n]));
        }
        Ok(n)
    }

    fn flush(&mut self) -> io::Result<()> {
        self.write.flush()
    }
}

fn main() -> ! {
    let result = main_sub();
    let exit_code = match result {
        Ok(()) => 0,
        Err(err) => {
            eprintln!("{}", err);
            err.exit_code()
        }
    };
    exit(exit_code);
}
