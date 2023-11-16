// use std::collections::HashMap;
//
// use cggmp_threshold_ecdsa::refresh;
// use cggmp_threshold_ecdsa::refresh::state_machine;
// use curv::elliptic::curves::Secp256k1;
// use round_based::{Msg, StateMachine};
// use wasm_bindgen::prelude::*;
//
// use mpc_protocol::Parameters;
//
// use crate::gg_2020::state_machine::keygen::LocalKey;
// use crate::round::RoundMsg;
//
// /// Key refresh.
// #[wasm_bindgen]
// pub struct KeyRefresh {
//     inner: refresh::state_machine::KeyRefresh,
// }
//
// #[wasm_bindgen]
// impl KeyRefresh {
//     /// Create a key refresh.
//     #[wasm_bindgen(constructor)]
//     pub fn new(
//         parameters: JsValue,
//         local_key: JsValue,
//         new_party_index: JsValue,
//         old_to_new: JsValue,
//         current_t: JsValue,
//     ) -> Result<KeyRefresh, JsError> {
//         let params: Parameters = serde_wasm_bindgen::from_value(parameters)?;
//         let local_key: Option<LocalKey<Secp256k1>> =
//             serde_wasm_bindgen::from_value(local_key)?;
//         let new_party_index: Option<u16> =
//             serde_wasm_bindgen::from_value(new_party_index)?;
//         let old_to_new: HashMap<u16, u16> =
//             serde_wasm_bindgen::from_value(old_to_new)?;
//         let current_t_option: Option<u16> =
//             serde_wasm_bindgen::from_value(current_t)?;
//
//         Ok(Self {
//             inner: state_machine::KeyRefresh::new(
//                 local_key,
//                 new_party_index,
//                 &old_to_new,
//                 params.threshold,
//                 params.parties,
//                 current_t_option,
//             )?,
//         })
//     }
//
//     /// Handle an incoming message.
//     #[wasm_bindgen(js_name = "handleIncoming")]
//     pub fn handle_incoming(
//         &mut self,
//         message: JsValue,
//     ) -> Result<(), JsError> {
//         let message: Msg<
//             <state_machine::KeyRefresh as StateMachine>::MessageBody,
//         > = serde_wasm_bindgen::from_value(message)?;
//         self.inner.handle_incoming(message)?;
//         Ok(())
//     }
//
//     /// Proceed to the next round.
//     pub fn proceed(&mut self) -> Result<JsValue, JsError> {
//         self.inner.proceed()?;
//         let messages = self.inner.message_queue().drain(..).collect();
//         let round = self.inner.current_round();
//         let messages = RoundMsg::from_round(round, messages);
//         Ok(serde_wasm_bindgen::to_value(&(round, &messages))?)
//     }
//
//     /// Get the key share.
//     pub fn create(&mut self) -> Result<JsValue, JsError> {
//         let local_key = self.inner.pick_output().unwrap()?;
//         let key_share: KeyShare = local_key.into();
//         Ok(serde_wasm_bindgen::to_value(&key_share)?)
//     }
// }
