use crate::error::GameError;
use crate::game::Game;
use crate::operation::Operation;

mod game;
mod error;
mod board;
mod operation;

/// Base class for tile types, provides methods needed bu the board to display and check the array of tiles
pub trait Tile {
    /// Check if this item is equivalent to the blank object for the current type
    fn is_blank(&self) -> bool;

    /// Return a displayable string for this tile object
    fn display_value(&self) -> String;

    /// Get the position this tile needs to be in to be considered 'solved'
    fn get_solved_pos(&self) -> usize;
}

impl Tile for u8 {
    fn is_blank(&self) -> bool {
        *self == 0
    }

    fn display_value(&self) -> String {
        if self.is_blank() {
            "".to_owned()
        } else {
            format!("{}", self)
        }
    }

    fn get_solved_pos(&self) -> usize {
        if self.is_blank() {
            15
        } else {
            (self - 1) as usize
        }
    }
}

/// Main game loop, prints the into message and loops while the game is not finished
fn main() -> Result<(), GameError> {
    println!("Welcome to 15 Puzzle! Your generated puzzle is below.");
    let mut game = Game::new();
    loop {
        println!("{game}");
        if game.is_done() {
            println!("Congratulations! You finished the game in {} moves!", game.moves());
            return Ok(());
        }
        println!("Enter w, a, s, or d to move the tile in the respective direction...");
        game.process_operation(Operation::get_next_from_stdin()?);
    }
}