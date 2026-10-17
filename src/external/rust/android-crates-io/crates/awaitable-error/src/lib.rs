use std::{error, fmt};

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {
    /// Awaitable is not initialized yet.
    Uninitialized,

    /// Awaitable is already consumed but not yet reset.
    AlreadyConsumed,

    /// Awaitable is marked done twice.
    AlreadyDone,
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        f.write_str(match self {
            Uninitialized => "Awaitable is not initialized yet.",
            AlreadyConsumed => "Awaitable is already consumed but not yet reset.",
            AlreadyDone => "Awaitable is marked done twice.",
        })
    }
}

impl error::Error for Error {}
