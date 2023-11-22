//! Key generation bindings simulated
use wasm_bindgen::{JsError, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;
use mpc_driver::gg2020_old::simulate::keygen::keygen_simulated_impl;

/// Bindings to simulation based key generation bindings
#[wasm_bindgen(js_name = "keygenSimulated")]
pub async fn keygen_simulated(parameters: JsValue) -> Result<JsValue, JsError> {
    let keyshares = keygen_simulated_impl(serde_wasm_bindgen::from_value(parameters)?).await?;
    Ok(serde_wasm_bindgen::to_value(&keyshares)?)
}
