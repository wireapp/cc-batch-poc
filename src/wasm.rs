use std::sync::Arc;

use crate::central::{Central, Context};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type CoreCryptoCommand;

    #[wasm_bindgen(structural, method, catch)]
    pub async fn execute(this: &CoreCryptoCommand, ctx: CoreCryptoContext) -> Result<(), JsValue>;
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub struct CoreCrypto {
    central: Central,
}

#[derive(Debug, thiserror::Error)]
pub enum CCError {
    #[error("Error {0}")]
    Error(String),
}

impl From<CCError> for wasm_bindgen::JsValue {
    fn from(val: CCError) -> Self {
        js_sys::Error::new(&val.to_string()).into()
    }
}

#[wasm_bindgen]
#[derive(Clone)]
pub struct CoreCryptoContext {
    context: Arc<Context>,
}

#[wasm_bindgen]
impl CoreCrypto {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            central: Central::new(),
        }
    }

    pub async fn transaction(&self, command: CoreCryptoCommand) -> Result<(), CCError> {
        let context = CoreCryptoContext {
            context: Arc::new(self.central.new_context().await),
        };
        log("executing command");
        let result = command.execute(context.clone()).await;
        log("commiting");
        if result.is_ok() {
            log("great success");
            context.context.finish().await;
        }
        Ok(())
    }
}

#[wasm_bindgen]
impl CoreCryptoContext {
    pub async fn decrypt(&self, conv_id: String, msg: Vec<u8>) -> Result<String, CCError> {
        log("decrypting");
        self.context
            .decrypt(&conv_id, &msg)
            .await
            .map_err(|e| CCError::Error(e.into()))
    }
}
