use std::io;
use std::io::Read;

use crate::error::GameError;

#[derive(Debug, PartialEq)]
pub enum Operation {
    Up,
    Down,
    Left,
    Right,
}

impl Operation {
    /// Return an operation from a code (if valid), or 'None' if invalid
    pub fn from_code(code: char) -> Option<Self> {
        match code {
            'w' => Some(Operation::Up),
            'a' => Some(Operation::Left),
            's' => Some(Operation::Down),
            'd' => Some(Operation::Right),
            _ => None
        }
    }

    /// Return the next operation from the given reader type
    pub fn get_next<R: Read>(reader: &mut R) -> Result<Operation, GameError> {
        loop {
            if let Some(Ok(byte)) = reader.by_ref().bytes().next() {
                // Check if we get an exit (CTRL + C) code as this isn't automatically handled in
                // raw mode
                if byte == 3 {
                    return Err(GameError::Exit);
                }
                match Self::from_code(byte as char) {
                    Some(op) => return Ok(op),
                    None => continue,
                }
            }
        };
    }

    /// Get the next operation from stdin (handles terminal swap to raw mode)
    pub fn get_next_from_stdin() -> Result<Operation, GameError> {
        // Raw mode allows us to get a single char as input so we don't need to wait for the
        // character + newline
        crossterm::terminal::enable_raw_mode()
            .map_err(GameError::from)?;
        let op = Self::get_next(&mut io::stdin());
        // Disable raw mode after reading the byte as it also changes general output behavior
        // which we don't want
        crossterm::terminal::disable_raw_mode().map_err(GameError::from)?;
        op
    }
}

#[test]
fn test_operation_left() {
    assert_eq!(Operation::from_code('w'), Some(Operation::Up));
    let next = Operation::get_next(&mut "w".as_bytes());
    assert!(next.is_ok());
    assert_eq!(next.unwrap(), Operation::Up);
}

#[test]
fn test_operation_right() {
    assert_eq!(Operation::from_code('a'), Some(Operation::Left));
    let next = Operation::get_next(&mut "a".as_bytes());
    assert!(next.is_ok());
    assert_eq!(next.unwrap(), Operation::Left);
}

#[test]
fn test_operation_up() {
    assert_eq!(Operation::from_code('s'), Some(Operation::Down));
    let next = Operation::get_next(&mut "s".as_bytes());
    assert!(next.is_ok());
    assert_eq!(next.unwrap(), Operation::Down);
}

#[test]
fn test_operation_down() {
    assert_eq!(Operation::from_code('d'), Some(Operation::Right));
    let next = Operation::get_next(&mut "d".as_bytes());
    assert!(next.is_ok());
    assert_eq!(next.unwrap(), Operation::Right);
}

#[test]
fn test_invalid_operation() {
    assert_eq!(Operation::from_code(';'), None);
}