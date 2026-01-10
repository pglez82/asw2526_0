use crate::{Coordinates, GameAction, PlayerId};
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum Movement {
    Placement {
        player: PlayerId,
        coords: Coordinates,
    },
    Action {
        player: PlayerId,
        action: GameAction,
    },
}

impl Display for Movement {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Movement::Placement { player, coords } => {
                write!(f, "Player {} places at {}", player, coords)
            }
            Movement::Action { player, action } => {
                write!(f, "Player {} performs action {}", player, action)
            }
        }
    }
}
