use std::{collections::HashMap, sync::Arc};

use axum::async_trait;
use tokio::sync::RwLock;

#[async_trait]
pub trait Repository {
    async fn get(&self, key: &str) -> Option<String>;
    async fn set(&self, key: &str, value: &str);
    async fn get_all(&self) -> HashMap<String, String>;
}

#[derive(Clone)]
pub struct InMemoryRepository {
    urls: Arc<RwLock<HashMap<String, String>>>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self {
            urls: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}

#[async_trait]
impl Repository for InMemoryRepository {
    async fn get(&self, key: &str) -> Option<String> {
        let urls = self.urls.read().await;
        urls.get(key).cloned()
    }

    async fn set(&self, key: &str, value: &str) {
        let mut urls = self.urls.write().await;
        urls.insert(key.to_string(), value.to_string());
    }

    async fn get_all(&self) -> HashMap<String, String> {
        let urls = self.urls.read().await;
        urls.clone()
    }
}
