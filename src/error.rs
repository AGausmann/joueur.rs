use std::error::Error as StdError;
use std::fmt;
use std::io::{self, ErrorKind};

use crate::client::exit::Exit;

use serde_json::error::Category;

#[derive(Debug)]
pub struct Error {
    source: Option<Box<dyn StdError + 'static>>,
    exit: Option<Exit>,
}

impl Error {
    pub fn from_exit(exit: Exit) -> Error {
        Error {
            source: None,
            exit: Some(exit),
        }
    }

    pub fn from_error<E>(err: E) -> Error
    where
        E: Into<Box<dyn StdError + 'static>>,
    {
        let err = err.into();
        let mut exit = None;
        if let Some(io_error) = err.downcast_ref::<io::Error>() {
            match io_error.kind() {
                ErrorKind::ConnectionReset
                | ErrorKind::ConnectionAborted
                | ErrorKind::BrokenPipe => exit = Some(Exit::DisconnectedUnexpectedly),
                ErrorKind::ConnectionRefused | ErrorKind::NotConnected => {
                    exit = Some(Exit::CouldNotConnect)
                }
                ErrorKind::TimedOut => exit = Some(Exit::ServerTimeout),
                _ => exit = Some(Exit::CannotReadSocket),
            }
        } else if let Some(json_error) = err.downcast_ref::<serde_json::Error>() {
            match json_error.classify() {
                Category::Io | Category::Eof => {
                    return Error::from_error(io::Error::from(
                        *err.downcast::<serde_json::Error>().unwrap(),
                    ))
                }
                Category::Syntax => exit = Some(Exit::MalformedJson),
                Category::Data => exit = Some(Exit::UnknownEventFromServer),
            }
        } else if let Some(_clap_error) = err.downcast_ref::<clap::Error>() {
            exit = Some(Exit::InvalidArgs);
        }
        Error {
            source: Some(err),
            exit,
        }
    }

    pub fn with_exit(self, exit: Exit) -> Self {
        Error {
            exit: Some(exit),
            ..self
        }
    }

    pub fn exit(&self) -> Option<Exit> {
        self.exit
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

impl<E> From<(Exit, E)> for Error
where
    E: Into<Error>,
{
    fn from((exit, err): (Exit, E)) -> Error {
        err.into().with_exit(exit)
    }
}

macro_rules! from_error {
    ($($t:ty),*) => {$(
        impl From<$t> for Error {
            fn from(err: $t) -> Error {
                Error::from_error(err)
            }
        }
    )*};
    ($($t:ty,)*) => { from_error! { $($t),* } };
}

from_error! {
    clap::Error,
    serde_json::Error,
    std::array::TryFromSliceError,
    std::cell::BorrowError,
    std::cell::BorrowMutError,
    std::char::CharTryFromError,
    std::char::DecodeUtf16Error,
    std::char::ParseCharError,
    std::env::JoinPathsError,
    std::env::VarError,
    std::ffi::FromBytesWithNulError,
    std::ffi::IntoStringError,
    std::ffi::NulError,
    std::fmt::Error,
    std::io::Error,
    std::net::AddrParseError,
    std::num::ParseFloatError,
    std::num::ParseIntError,
    std::num::TryFromIntError,
    std::path::StripPrefixError,
    std::str::ParseBoolError,
    std::str::Utf8Error,
    std::string::FromUtf16Error,
    std::string::FromUtf8Error,
    std::string::ParseError,
    std::sync::mpsc::RecvError,
    std::sync::mpsc::RecvTimeoutError,
    std::sync::mpsc::TryRecvError,
    std::thread::AccessError,
    std::time::SystemTimeError,
    String,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match (&self.exit, &self.source) {
            (Some(exit), Some(source)) => write!(f, "{}: {}", exit, source),
            (Some(exit), None) => write!(f, "{}", exit),
            (None, Some(source)) => write!(f, "{}", source),
            (None, None) => unreachable!("some error reason must be given"),
        }
    }
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        self.source.as_ref().map(Box::as_ref)
    }
}
