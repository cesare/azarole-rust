use anyhow::Result;

use crate::app::config::ApplicationConfig;

#[derive(Clone)]
pub struct ApplicationContext {
    pub config: ApplicationConfig,
}

impl ApplicationContext {
    pub fn new(config: &ApplicationConfig) -> Result<Self> {
        let context = Self {
            config: config.clone(),
        };
        Ok(context)
    }
}
