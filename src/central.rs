use std::{collections::HashMap, ops::Deref, sync::Arc};

use async_lock::RwLock;

use crate::keystore::{KeyStore, KeyStoreTransaction};

pub struct Conversation {
    members: Vec<String>,
}

#[derive(Clone)]
pub struct Central {
    keystore: KeyStore,
    conversations: Arc<RwLock<HashMap<String, RwLock<Conversation>>>>,
}

pub struct Context {
    central: Central,
    transaction: KeyStoreTransaction,
}

impl Central {
    pub fn new() -> Self {
        let mut conversations = HashMap::new();
        conversations.insert(
            "conv1".into(),
            RwLock::new(Conversation { members: vec![] }),
        );
        Self {
            keystore: KeyStore::new(),
            conversations: Arc::new(RwLock::new(conversations)),
        }
    }

    pub async fn new_context(&self) -> Context {
        Context {
            central: self.clone(),
            transaction: self.keystore.new_transaction().await,
        }
    }

    // this would be a function that would operate outside of the context
    pub async fn conversation_epoch(&self, id: &str) -> Result<u64, &'static str> {
        self.conversations
            .read()
            .await
            .get(id)
            .ok_or("Not Found")?
            .read()
            .await;
        Ok(42)
    }
}

/// this would allow the context to access its parent members for better ergonomics
impl Deref for Context {
    type Target = Central;

    fn deref(&self) -> &Self::Target {
        &self.central
    }
}

impl Context {
    pub async fn decrypt(&self, conv_id: &str, msg: &[u8]) -> Result<String, &'static str> {
        println!("decrypting");
        self.central
            .conversations
            .read()
            .await
            .get(conv_id)
            .ok_or("Not Found")?
            .write()
            .await
            .decrypt(msg)
    }

    pub async fn generate_keypackages(&mut self) -> Result<Vec<u8>, &'static str> {
        self.transaction.add("key_packages".into(), vec![]).await;
        Ok(vec![])
    }

    /// #WARNING: this can be called only once and not used afterwards
    /// the reason it is not dropped is uniffi
    /// in the final implementation we should definitely check and return an error if it was
    /// already committed
    pub async fn finish(&self) {
        self.transaction.commit().await.unwrap();
    }
}

impl Default for Central {
    fn default() -> Self {
        Self::new()
    }
}

impl Conversation {
    pub fn decrypt(&mut self, msg: &[u8]) -> Result<String, &'static str> {
        let result = String::from_utf8_lossy(msg).into_owned();
        println!("message: {result}");
        Ok(result)
    }
}
