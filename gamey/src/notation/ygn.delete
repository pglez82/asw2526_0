use serde::{Deserialize, Serialize};

const DEFAULT_BOARD_SIZE: u32 = 7;
const DEFAULT_NUM_PLAYERS: u32 = 2;

#[derive(Serialize, Deserialize, Debug)]
pub struct YGN {
    pub config: Config,
    pub history: Vec<Move>,
}

impl YGN {
    pub fn new() -> Self {
        Self {
            config: Config::default(),
            history: Vec::new(),
        }
    }
}

impl Default for YGN {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Config {
    pub size: u32,
    pub num_players: u32,
    pub variant: Variant,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            size: DEFAULT_BOARD_SIZE,
            num_players: DEFAULT_NUM_PLAYERS,
            variant: Variant::default(),
        }
    }
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum Move {
    Placement {
        player: u32,      // PlayerId
        coords: Vec<u32>, // Coordinates
    },
    Action {
        player: u32, // PlayerId
        action: Action,
    },
}

/// Possible actions a player can take
#[derive(Serialize, Deserialize, Debug)]
pub enum Action {
    Swap,
    Resign,
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub enum Variant {
    #[default]
    Standard,
    // Other variants can be added here
}
