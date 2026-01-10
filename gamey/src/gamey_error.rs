use thiserror::Error;

use crate::{Coordinates, Movement, PlayerId};
#[derive(Debug, Error)]
pub enum GameYError {
    #[error("I/O error {message}: {error}")]
    IoError { message: String, error: String },

    #[error("Serde JSON error: {error}")]
    SerdeError { error: serde_json::Error },

    #[error("Invalid coordinates: expected {expected} coords, found {found}")]
    BadCoordsNumber { expected: usize, found: usize },

    #[error("Coordinate {id_coord}={coord} is out of range for board size {board_size}")]
    CoordOutOfRange {
        id_coord: char,
        coord: u32,
        board_size: u32,
    },

    #[error("Player {player} tries to place a stone on an occupied position: {coordinates}")]
    Occupied {
        coordinates: Coordinates,
        player: PlayerId,
    },

    #[error("Invalid character '{char}' in layout at row {row}, column {col}")]
    InvalidCharInLayout { char: char, row: usize, col: usize },

    #[error("Attempt to play movement {movement} in a finished game")]
    GameOver { movement: Movement },

    #[error("Wrong player in movement: Expected player {expected}, found player {found}")]
    InvalidPlayerTurn { expected: PlayerId, found: PlayerId },

    #[error("Invalid number of players: {num_players}, expected {expected}")]
    InvalidNumPlayers { num_players: u32, expected: u32 },

    #[error("Invalid YEN layout: expected {expected} rows, found {found} rows")]
    InvalidYENLayout { expected: u32, found: u32 },

    #[error("Invalid YEN layout line: expected {expected} rows, found {found} rows at line {line}")]
    InvalidYENLayoutLine {
        expected: u32,
        found: u32,
        line: u32,
    },
}
