// use wasm_bindgen::{JsError, JsValue};
// use wasm_bindgen::prelude::wasm_bindgen;
//
// use mpc_protocol::Parameters;
//
// use crate::gg2020_old::simulate::simulation::Simulation;
// use crate::gg20::KeyShare;
// use crate::gg_2020::state_machine::keygen::Keygen;
//
// #[wasm_bindgen(js_name = "keygenSimulated")]
// pub fn keygen_simulated(parameters: JsValue) -> Result<JsValue, JsError> {
//     let params: Parameters = serde_wasm_bindgen::from_value(parameters)?;
//     let t = params.threshold;
//     let n = params.parties;
//     let mut simulation = Simulation::<Keygen>::new();
//
//     for i in 1..=n {
//         simulation.add_party(Keygen::new(i, t, n).unwrap());
//     }
//
//     let keys = simulation.run().unwrap();
//
//     let key_shares: Vec<KeyShare> =
//         keys.into_iter().map(|k| k.into()).collect();
//     Ok(serde_wasm_bindgen::to_value(&key_shares)?)
// }
