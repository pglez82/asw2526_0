use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct YEN {
    size: u32,
    turn: u32,
    players: Vec<char>,
    /// A compact string representation of the board
    /// Example: "B/..R/.B.R"
    layout: String,
}

impl YEN {
    pub fn new(size: u32, turn: u32, players: Vec<char>, layout: String) -> Self {
        YEN {
            size,
            turn,
            players,
            layout,
        }
    }

    pub fn layout(&self) -> &str {
        &self.layout
    }

    pub fn size(&self) -> u32 {
        self.size
    }

    pub fn turn(&self) -> u32 {
        self.turn
    }
}
