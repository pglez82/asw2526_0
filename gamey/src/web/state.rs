use crate::YBotRegistry;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    bots: Arc<YBotRegistry>,
}

impl AppState {
    pub fn new(bots: YBotRegistry) -> Self {
        Self {
            bots: Arc::new(bots),
        }
    }

    pub fn bots(&self) -> Arc<YBotRegistry> {
        Arc::clone(&self.bots)
    }
}
