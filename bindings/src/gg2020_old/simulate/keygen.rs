//! Key generation bindings simulated
use wasm_bindgen::{JsError, JsValue};
use wasm_bindgen::prelude::wasm_bindgen;

/// Bindings to simulation based key generation bindings
#[wasm_bindgen(js_name = "keygenSimulated")]
pub fn keygen_simulated(parameters: JsValue) -> Result<JsValue, JsError> {
    Ok(serde_wasm_bindgen::to_value(&serde_wasm_bindgen::from_value(parameters)?)?)
}
