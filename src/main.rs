use std::env;
use std::io::{self, BufReader, BufWriter, Read, Write};
use std::net::TcpStream;
use std::process::exit;

use joueur::client::exit::Exit;
use joueur::client::proto::{ClientEvent, EventSink, EventStream, ServerEvent};
use joueur::error::Error;
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
    let args = Args::from_iter_safe(env::args_os())?;

    let mut server_parts = args.server.split(":");
    let server_name = server_parts.next().unwrap();
    let server_port = match server_parts.next() {
        Some(port_str) => port_str
            .parse()
            .map_err(|e| Error::from_error(e).with_exit(Exit::InvalidArgs))?,
        None => args.port,
    };

    let socket = TcpStream::connect((server_name, server_port))?;
    let read_half = socket.try_clone()?;
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
            err.exit().map(|ex| ex.code()).unwrap_or(1)
        }
    };
    exit(exit_code);
}
