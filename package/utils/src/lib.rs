use std::convert::From;
use std::io;

#[derive(Debug)]
pub enum Error {
    KeyNotFound,
    KeyAlreadyExists,
    UnexpectedError,
    KeyOverflowError,
    ValueOverflowError,
    TryFromSliceError(&'static str),
    UTF8Error,
}

impl From<io::Error> for Error {
    fn from(_e: io::Error) -> Error {
        Error::UnexpectedError
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
