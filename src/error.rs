use std::error::Error as StdError;

pub struct Error {}

impl<E> From<E> for Error
where
    E: StdError + 'static,
{
    fn from(err: E) -> Error {
        Error {}
    }
}
