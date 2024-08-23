use std::{
    collections::{HashMap, HashSet},
    sync::Arc,
};

use async_lock::RwLock;

#[derive(Clone)]
pub struct KeyStore {
    /// interion mutability is used here to illustrate how typically connections to storages are
    /// implemented
    db: Arc<RwLock<HashMap<String, Vec<u8>>>>,
}

pub struct KeyStoreTransaction {
    keystore: KeyStore,
    to_add_update: RwLock<HashMap<String, Vec<u8>>>,
    to_remove: RwLock<HashSet<String>>,
}

impl KeyStore {
    pub fn new() -> Self {
        Self {
            db: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn add(&self, key: &str, value: Vec<u8>) {
        self.db.write().await.insert(key.into(), value);
    }

    pub async fn remove_all<T: Iterator<Item = K>, K: AsRef<str>>(
        &self,
        to_remove: T,
    ) -> Result<(), &'static str> {
        for k in to_remove {
            self.db.write().await.remove(k.as_ref());
        }
        Ok(())
    }

    pub async fn upsert<T: Iterator<Item = (String, Vec<u8>)>>(
        &self,
        to_add_update: T,
    ) -> Result<(), &'static str> {
        self.db.write().await.extend(to_add_update);
        Ok(())
    }

    pub async fn new_transaction(&self) -> KeyStoreTransaction {
        KeyStoreTransaction {
            keystore: self.clone(),
            to_add_update: HashMap::new().into(),
            to_remove: HashSet::new().into(),
        }
    }
}

impl KeyStoreTransaction {
    pub async fn commit(&self) -> Result<(), &'static str> {
        self.keystore
            .remove_all(self.to_remove.write().await.iter())
            .await?;
        self.keystore
            .upsert(self.to_add_update.write().await.drain())
            .await?;
        println!("commit!");
        Ok(())
    }

    pub async fn add(&mut self, key: String, value: Vec<u8>) {
        self.to_add_update.write().await.insert(key, value);
    }
}
