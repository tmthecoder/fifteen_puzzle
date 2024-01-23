use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io;

#[derive(Debug)]
pub enum GameError {
    Exit,
    Other(Box<dyn Error>),
}

impl Display for GameError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exit => write!(f, "Exiting..."),
            Self::Other(e) => write!(f, "Exiting with unexpected error: {}", e),
        }
    }
}

impl Error for GameError {}

impl From<io::Error> for GameError {
    fn from(value: io::Error) -> Self {
        GameError::Other(Box::new(value))
    }
}