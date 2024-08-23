#![allow(dead_code)]

use std::sync::Arc;

use self::central::{Central, Context};

pub mod central;
mod keystore;
#[cfg(target_family = "wasm")]
pub mod wasm;

uniffi::setup_scaffolding!();

#[derive(uniffi::Object)]
pub struct CoreCrypto {
    central: Central,
}

#[derive(Debug, uniffi::Enum, thiserror::Error)]
pub enum CCError {
    #[error("Error {0}")]
    Error(String),
}

#[derive(uniffi::Object)]
pub struct CoreCryptoContext {
    context: Context,
}

#[uniffi::export(with_foreign)]
#[async_trait::async_trait]
pub trait CoreCryptoCommand: std::fmt::Debug + Send + Sync {
    /// Will be called inside a transaction in CoreCrypto
    async fn execute(&self, context: Arc<CoreCryptoContext>) -> Result<(), CCError>;
}

#[uniffi::export]
impl CoreCrypto {
    #[uniffi::constructor]
    pub fn new() -> Self {
        Self {
            central: Central::new(),
        }
    }

    pub async fn transaction(&self, command: Arc<dyn CoreCryptoCommand>) -> Result<(), CCError> {
        let context = Arc::new(CoreCryptoContext {
            context: self.central.new_context().await,
        });
        let result = command.execute(context.clone()).await;
        println!("commiting");
        if result.is_ok() {
            context.context.finish().await;
        }
        Ok(())
    }
}

impl Default for CoreCrypto {
    fn default() -> Self {
        Self::new()
    }
}

#[uniffi::export]
impl CoreCryptoContext {
    pub async fn decrypt(&self, conv_id: String, msg: Vec<u8>) -> Result<String, CCError> {
        self.context
            .decrypt(&conv_id, &msg)
            .await
            .map_err(|e| CCError::Error(e.into()))
    }
}
