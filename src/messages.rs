use std::{collections::HashMap, sync::Arc};

use rust_extensions::date_time::DateTimeAsMicroseconds;
use tokio::sync::RwLock;

pub struct SecretMessage {
    pub id: String,
    pub expires: DateTimeAsMicroseconds,
    pub message: String,
    pub ips: Vec<String>,
}

impl SecretMessage {
    pub fn has_ip(&self, my_ip: &str) -> bool {
        for ip in &self.ips {
            if ip == my_ip {
                return true;
            }
        }

        false
    }
}

pub struct SecretMessages {
    item: RwLock<HashMap<String, Arc<SecretMessage>>>,
}

impl SecretMessages {
    pub fn new() -> Self {
        Self {
            item: RwLock::new(HashMap::new()),
        }
    }

    pub async fn add(&self, message: SecretMessage) {
        let mut messages = self.item.write().await;
        messages.insert(message.id.to_string(), Arc::new(message));
    }

    pub async fn get(&self, id: &str) -> Option<Arc<SecretMessage>> {
        let messages = self.item.read().await;
        messages.get(id).cloned()
    }
}
