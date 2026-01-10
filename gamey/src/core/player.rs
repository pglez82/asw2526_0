use std::fmt::Display;

/// Representation of a player in the game.
#[derive(Debug, Clone)]
pub struct Player {
    id: PlayerId,
    name: String,
}

impl Player {
    pub fn new(id: PlayerId, name: String) -> Self {
        Self { id, name }
    }

    pub fn id(&self) -> PlayerId {
        self.id
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Display for Player {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Player {}: {}", self.id, self.name)
    }
}

// Wrapper for player identifiers.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct PlayerId(u32);

impl PlayerId {
    pub fn new(id: u32) -> Self {
        Self(id)
    }

    pub fn id(&self) -> u32 {
        self.0
    }
}

impl Display for PlayerId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
