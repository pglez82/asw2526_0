use std::fmt::Display;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameAction {
    Swap,
    Resign,
}

impl Display for GameAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameAction::Swap => write!(f, "Swap"),
            GameAction::Resign => write!(f, "Resign"),
        }
    }
}
