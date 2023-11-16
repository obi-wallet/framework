//! Message signing.
//
// use std::convert::TryInto;
//
// use curv::BigInt;
// use curv::elliptic::curves::Secp256k1;
// use round_based::{Msg, StateMachine};
// use wasm_bindgen::prelude::*;
//
// use crate::gg2020_old::utils;
// use crate::gg20::Signature;
// use crate::gg_2020::party_i::verify;
// use crate::gg_2020::state_machine::keygen::LocalKey;
// use crate::gg_2020::state_machine::sign::{CompletedOfflineStage, OfflineStage, PartialSignature, SignManual};
// use crate::round::RoundMsg;
//
// pub const ERR_COMPLETED_OFFLINE_STAGE: &str =
//     "completed offline stage unavailable, has partial() been called?";
//
// /// Round-based signing protocol.
// #[wasm_bindgen]
// pub struct Signer {
//     inner: OfflineStage,
//     completed: Option<(CompletedOfflineStage, BigInt)>,
// }
//
// #[wasm_bindgen]
// impl Signer {
//     /// Create a signer.
//     #[wasm_bindgen(constructor)]
//     pub fn new(
//         index: JsValue,
//         participants: JsValue,
//         local_key: JsValue,
//     ) -> Result<Signer, JsError> {
//         let index: u16 = serde_wasm_bindgen::from_value(index)?;
//         let participants: Vec<u16> =
//             serde_wasm_bindgen::from_value(participants)?;
//         let local_key: LocalKey<Secp256k1> =
//             serde_wasm_bindgen::from_value(local_key)?;
//         Ok(Signer {
//             inner: OfflineStage::new(index, participants.clone(), local_key)?,
//             completed: None,
//         })
//     }
//
//     /// Handle an incoming message.
//     #[wasm_bindgen(js_name = "handleIncoming")]
//     pub fn handle_incoming(
//         &mut self,
//         message: JsValue,
//     ) -> Result<(), JsError> {
//         let message: Msg<<OfflineStage as StateMachine>::MessageBody> =
//             serde_wasm_bindgen::from_value(message)?;
//
//         self.inner.handle_incoming(message)?;
//         Ok(())
//     }
//
//     /// Proceed to the next round.
//     pub fn proceed(&mut self) -> Result<JsValue, JsError> {
//         if self.inner.wants_to_proceed() {
//             self.inner.proceed()?;
//             let messages = self.inner.message_queue().drain(..).collect();
//             let round = self.inner.current_round();
//             let messages = RoundMsg::from_round(round, messages);
//             Ok(serde_wasm_bindgen::to_value(&(round, &messages))?)
//         } else {
//             Ok(serde_wasm_bindgen::to_value(&false)?)
//         }
//     }
//
//     /// Returns the completed offline stage if available.
//     pub fn completed_offline_stage(&mut self) -> Result<JsValue, JsError> {
//         let completed_offline_stage = self.inner.pick_output().unwrap()?;
//         Ok(serde_wasm_bindgen::to_value(&completed_offline_stage)?)
//     }
//
//     /// Generate the completed offline stage and store the result
//     /// internally to be used when `create()` is called.
//     ///
//     /// Return a partial signature that must be sent to the other
//     /// signing participants.
//     pub fn partial(&mut self, message: JsValue) -> Result<JsValue, JsError> {
//         let message: Vec<u8> = serde_wasm_bindgen::from_value(message)?;
//         let message: [u8; 32] = message.as_slice().try_into()?;
//         let completed_offline_stage = self.inner.pick_output().unwrap()?;
//         let data = BigInt::from_bytes(&message);
//         let (_sign, partial) =
//             SignManual::new(data.clone(), completed_offline_stage.clone())?;
//
//         self.completed = Some((completed_offline_stage, data));
//
//         Ok(serde_wasm_bindgen::to_value(&partial)?)
//     }
//
//     /// Add partial signatures without validating them. Allows multiple partial signatures
//     /// to be combined into a single partial signature before sending it to the other participants.
//     pub fn add(&mut self, partials: JsValue) -> Result<JsValue, JsError> {
//         let partials: Vec<PartialSignature> =
//             serde_wasm_bindgen::from_value(partials)?;
//
//         let (completed_offline_stage, data) = self
//             .completed
//             .take()
//             .ok_or_else(|| JsError::new(ERR_COMPLETED_OFFLINE_STAGE))?;
//         let (sign, _partial) =
//             SignManual::new(data.clone(), completed_offline_stage.clone())?;
//
//         let (_sign, aggregated_partial) = sign.add(&partials)?;
//
//         Ok(serde_wasm_bindgen::to_value(&aggregated_partial)?)
//     }
//
//     /// Create and verify the signature.
//     pub fn create(&mut self, partials: JsValue) -> Result<JsValue, JsError> {
//         let partials: Vec<PartialSignature> =
//             serde_wasm_bindgen::from_value(partials)?;
//
//         let (completed_offline_stage, data) = self
//             .completed
//             .take()
//             .ok_or_else(|| JsError::new(ERR_COMPLETED_OFFLINE_STAGE))?;
//         let pk = completed_offline_stage.public_key().clone();
//
//         let (sign, _partial) =
//             SignManual::new(data.clone(), completed_offline_stage.clone())?;
//
//         let signature = sign.complete(&partials)?;
//         verify(&signature, &pk, &data).map_err(|e| {
//             JsError::new(&format!("failed to verify signature: {:?}", e))
//         })?;
//
//         let public_key = pk.to_bytes(false).to_vec();
//         let result = Signature {
//             signature,
//             address: utils::address(&public_key),
//             public_key,
//         };
//
//         Ok(serde_wasm_bindgen::to_value(&result)?)
//     }
// }
//
