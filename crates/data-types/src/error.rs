use std::fmt;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    Plant(String),
    Other(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        use Error::*;

        match self {
            Plant(err) => err.fmt(f),
            Other(err) => err.fmt(f),
        }
    }
}

impl From<&str> for Error {
    fn from(v: &str) -> Self {
        Self::Other(v.to_owned())
    }
}
