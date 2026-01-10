use std::{collections::HashMap, sync::Arc};

use crate::YBot;

pub struct YBotRegistry {
    bots: HashMap<String, Arc<dyn YBot>>,
}

impl YBotRegistry {
    pub fn new() -> Self {
        YBotRegistry {
            bots: HashMap::new(),
        }
    }

    pub fn with_bot(mut self, bot: Arc<dyn YBot>) -> Self {
        self.bots.insert(bot.name().to_string(), bot);
        self
    }

    pub fn find(&self, name: &str) -> Option<Arc<dyn YBot>> {
        self.bots.get(name).cloned()
    }

    pub fn names(&self) -> Vec<String> {
        self.bots.keys().cloned().collect()
    }
}

impl Default for YBotRegistry {
    fn default() -> Self {
        YBotRegistry::new()
    }
}
