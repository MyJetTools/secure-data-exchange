use std::{collections::HashMap, sync::Arc};

use my_http_server::types::FileContent;
use rust_extensions::{date_time::DateTimeAsMicroseconds, lazy::LazyVec};
use tokio::sync::RwLock;

pub struct SecretFile {
    pub id: String,
    pub expires: DateTimeAsMicroseconds,
    pub ips: Vec<String>,
    pub file: FileContent,
}

impl SecretFile {
    pub fn has_ip(&self, my_ip: &str) -> bool {
        for ip in &self.ips {
            if ip == my_ip {
                return true;
            }
        }

        false
    }
}

pub struct SecretFiles {
    item: RwLock<HashMap<String, Arc<SecretFile>>>,
}

impl SecretFiles {
    pub fn new() -> Self {
        Self {
            item: RwLock::new(HashMap::new()),
        }
    }

    pub async fn add(&self, file: SecretFile) {
        let mut messages = self.item.write().await;
        messages.insert(file.id.to_string(), Arc::new(file));
    }

    pub async fn get(&self, id: &str) -> Option<Arc<SecretFile>> {
        let messages = self.item.read().await;
        messages.get(id).cloned()
    }

    pub async fn gc(&self, now: DateTimeAsMicroseconds) {
        let to_gc = {
            let mut to_gc = LazyVec::new();
            let messages = self.item.read().await;
            for (id, message) in messages.iter() {
                if message.expires.unix_microseconds < now.unix_microseconds {
                    to_gc.add(id.to_string());
                }
            }

            to_gc.get_result()
        };

        if let Some(to_gc) = to_gc {
            let mut messages = self.item.write().await;

            for id in &to_gc {
                messages.remove(id);
            }
        }
    }
}
