use crate::{Coordinates, GameY, YBot};
use rand::prelude::IndexedRandom;
pub struct RandomBot;

impl YBot for RandomBot {
    fn name(&self) -> &str {
        "random_bot"
    }
    fn choose_move(&self, board: &GameY) -> Option<Coordinates> {
        let available_cells = board.available_cells();
        let cell = available_cells.choose(&mut rand::rng())?;
        let coordinates = Coordinates::from_index(*cell, board.board_size());
        Some(coordinates)
    }
}
