use std::fmt::{Display, Formatter};

use crate::board::Board;
use crate::operation::Operation;
use crate::Tile;

/// The main game structure
/// Handles propagation of updates to the board structure, tracks the current state of the game, and the amount of
/// elapsed moves
pub struct Game<T: Tile> {
    board: Board<T>,
    current_state: GameState,
    move_count: usize,
}

/// The state of the game (either in progress or finished)
#[derive(PartialEq)]
enum GameState {
    InProgress,
    Finished,
}

impl Game<u8> {
    pub fn new() -> Self {
        Self::with_board(Board::new())
    }
}

impl<T: Tile> Game<T> {
    /// Create a new game with a custom board with the given Tile type
    pub fn with_board(board: Board<T>) -> Self {
        Self {
            board,
            current_state: GameState::InProgress,
            move_count: 0,
        }
    }

    /// Return whether the current state is equivalent to that of the finished state
    pub fn is_done(&self) -> bool {
        self.current_state == GameState::Finished
    }

    /// Return the current move count
    pub fn moves(&self) -> usize {
        self.move_count
    }

    /// Process a movement operation (propagates to the board & updates counter/state if applicable)
    pub fn process_operation(&mut self, operation: Operation) {
        // If this move resulted in an actual swap, update the counter
        if self.board.process_operation(operation) {
            self.move_count += 1;
        }
        // Update the state if the game is finished
        if self.board.is_solved() {
            self.current_state = GameState::Finished;
        }
    }
}

impl<T: Tile> Display for Game<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}", self.board)?;
        writeln!(f, "Move Count: {}", self.move_count)
    }
}

#[test]
fn test_is_done() {
    // New game should not be done (in an init/in-progress state)
    let game = Game::new();
    assert!(!game.is_done());

    // Test that the state updates and the game is shown as done after a dummy move on a complete board
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
    let board = Board::from_existing_array(array);
    let mut game = Game::with_board(board);
    game.process_operation(Operation::Left);
    assert!(game.is_done());
}

#[test]
fn test_process_operation() {
    // Test that a valid move (one that changes the board) updates the move counter
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
    let board = Board::from_existing_array(array);
    let mut game = Game::with_board(board);
    game.process_operation(Operation::Right);
    assert_eq!(game.move_count, 1);


    // Test that an invalid move does not update the move counter
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
    let board = Board::from_existing_array(array);
    let mut game = Game::with_board(board);
    game.process_operation(Operation::Left);
    assert_eq!(game.move_count, 0);
}