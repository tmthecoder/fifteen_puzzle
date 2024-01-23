use std::fmt::{Display, Formatter};

use rand::prelude::SliceRandom;

use crate::operation::Operation;
use crate::Tile;

pub struct Board<T: Tile> {
    array: [T; 16],
    blank_idx: usize,
}

impl<T: Tile> Display for Board<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let mut builder = tabled::builder::Builder::new();
        for i in 0..(self.array.len() / 4) {
            let start = i * 4;
            let row: Vec<String> = self.array[start..(start + 4)]
                .iter()
                .map(Tile::display_value).collect();
            builder.push_record(row);
        }
        let string = builder.build().to_string();
        write!(f, "{}", string)
    }
}

impl Board<u8> {
    /// Create a new board of shuffled u8 values
    pub fn new() -> Self {
        let mut rng = rand::thread_rng();
        let mut array: [u8; 16] = (0..16).collect::<Vec<u8>>().try_into().unwrap();
        loop {
            array.shuffle(&mut rng);
            let blank_idx = array.iter().position(Tile::is_blank).unwrap();
            if Self::is_solvable(&array, blank_idx) {
                break;
            }
        };
        // We can safely unwrap this as the array must contain a 0
        Self::from_existing_array(array)
    }
}

impl<T: Tile> Board<T> {
    /// Checks if the array contains the layout of a solvable puzzle.
    /// Referenced from https://www.geeksforgeeks.org/check-instance-15-puzzle-solvable/
    fn is_solvable(arr: &[T; 16], blank: usize) -> bool {
        let pos_from_bottom = 4 - blank / 4;
        let mut inversions = 0;
        for i in 0..arr.len() - 1 {
            for j in i + 1..arr.len() {
                if arr[i].get_solved_pos() > arr[j].get_solved_pos() {
                    inversions += 1;
                }
            }
        };

        (pos_from_bottom % 2 == 0 && inversions % 2 != 0) ||
            (pos_from_bottom % 2 != 0 && inversions % 2 == 0)
    }

    /// Create a board from an existing array of tiles
    pub fn from_existing_array(array: [T; 16]) -> Self {
        let blank_idx = array.iter().position(Tile::is_blank).unwrap();
        Self {
            array,
            blank_idx
        }
    }

    /// Process an operation and update the board if it is a valid operation
    pub fn process_operation(&mut self, operation: Operation) -> bool {
        let swap_offset = match operation {
            Operation::Up => 4,
            Operation::Down => -4,
            Operation::Left => 1,
            Operation::Right => -1,
        };

        let swap_idx = self.blank_idx as isize + swap_offset;
        if swap_idx < 0 || swap_idx as usize >= self.array.len() {
            return false;
        }

        // Edge case where the blank tile is on the left most edge and the user
        // sends a right swap
        if self.blank_idx % 4 == 0 && self.blank_idx as isize == swap_idx + 1 {
            return false;
        }

        // Edge case where the blank tile is on the right most edge and the user
        // sends a left swap
        if swap_idx % 4 == 0 && self.blank_idx as isize == swap_idx - 1 {
            return false;
        }

        self.array.swap(self.blank_idx, swap_idx as usize);

        self.blank_idx = swap_idx as usize;

        return true;
    }

    /// Return whether this board matches the layout of a solved board
    pub fn is_solved(&self) -> bool {
        self.array.iter().enumerate().all(|(idx, tile)| {
            idx == tile.get_solved_pos()
        })
    }
}

#[test]
fn test_is_solved() {
    // Provide a solved board
    let array = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
    let board = Board::from_existing_array(array);
    assert!(board.is_solved());

    // Provide an unsolved board
    let array = [2, 1, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0];
    let board = Board::from_existing_array(array);
    assert!(!board.is_solved())
}

#[test]
fn test_process_operation_up() {
    // Test an up operation (swaps blank with item below it)
    let array = [1, 2, 3, 4, 0, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 5];
    let final_array = [1, 2, 3, 4, 9, 6, 7, 8, 0, 10, 11, 12, 13, 14, 15, 5];
    let mut board = Board::from_existing_array(array);
    board.process_operation(Operation::Up);
    assert_eq!(board.array, final_array);
}

#[test]
fn test_process_operation_down() {
    // Test an up operation (swaps blank with item above it)
    let array = [1, 2, 3, 4, 0, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 5];
    let final_array = [0, 2, 3, 4, 1, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 5];
    let mut board = Board::from_existing_array(array);
    board.process_operation(Operation::Down);
    assert_eq!(board.array, final_array);
}

#[test]
fn test_process_operation_right() {
    // Test an up operation (swaps blank with item to the left of it)
    let array = [1, 2, 3, 0, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 5];
    let final_array = [1, 2, 0, 3, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 5];
    let mut board = Board::from_existing_array(array);
    board.process_operation(Operation::Right);
    assert_eq!(board.array, final_array);

    // Test the edge case when the item is on the left-most side
    let array = [1, 2, 3, 4, 0, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 5];
    let final_array = array.clone();
    let mut board = Board::from_existing_array(array);
    board.process_operation(Operation::Right);
    assert_eq!(board.array, final_array);
}

#[test]
fn test_process_operation_left() {
    // Test an up operation (swaps blank with item below it)
    let array = [1, 2, 3, 4, 0, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 5];
    let final_array = [1, 2, 3, 4, 6, 0, 7, 8, 9, 10, 11, 12, 13, 14, 15, 5];
    let mut board = Board::from_existing_array(array);
    board.process_operation(Operation::Left);
    assert_eq!(board.array, final_array);

    // Test the edge case when the item is on the right-most side
    let array = [1, 2, 3, 0, 4, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 5];
    let final_array = array.clone();
    let mut board = Board::from_existing_array(array);
    board.process_operation(Operation::Left);
    assert_eq!(board.array, final_array);
}