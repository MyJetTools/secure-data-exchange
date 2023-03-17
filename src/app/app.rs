use std::sync::Arc;

use rust_extensions::AppStates;

use crate::{files::SecretFiles, messages::SecretMessages};

pub struct AppContext {
    pub app_states: Arc<AppStates>,
    pub messages: SecretMessages,
    pub files: SecretFiles,
}

pub const APP_VERSION: &'static str = env!("CARGO_PKG_VERSION");
pub const APP_NAME: &'static str = env!("CARGO_PKG_NAME");

impl AppContext {
    pub async fn new() -> Self {
        let app_states = Arc::new(AppStates::create_initialized());

        Self {
            app_states: app_states.clone(),
            messages: SecretMessages::new(),
            files: SecretFiles::new(),
        }
    }
}
