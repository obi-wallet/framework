//! Key generation.

use cggmp_threshold_ecdsa::mpc_ecdsa::gg_2020::state_machine::keygen::Keygen;
use wasm_bindgen::prelude::*;

use mpc_protocol::Parameters;

use crate::gg2020_old::utils::PartySignup;
use crate::gg20::KeyShare;
use crate::round::RoundMsg;

/// Create a key generator.
pub fn new_keygen(
    parameters: Parameters,
    party_signup: PartySignup,
) -> Result<Keygen, JsError> {
    let PartySignup { number, uuid } = party_signup;
    let (party_num_int, _uuid) = (number, uuid);
    Ok(Keygen::new(
        party_num_int,
        parameters.threshold,
        parameters.parties,
    )?)
}


//
// impl KeyGenerator {
//
//     //
//     // /// Handle an incoming message.
//     // #[wasm_bindgen(js_name = "handleIncoming")]
//     // pub fn handle_incoming(
//     //     &mut self,
//     //     message: JsValue,
//     // ) -> Result<(), JsError> {
//     //     let message: Msg<<Keygen as StateMachine>::MessageBody> =
//     //         serde_wasm_bindgen::from_value(message)?;
//     //     self.inner.handle_incoming(message)?;
//     //     Ok(())
//     // }
//     //
//     // /// Proceed to the next round.
//     // pub fn proceed(&mut self) -> Result<JsValue, JsError> {
//     //     self.inner.proceed()?;
//     //     let messages = self.inner.message_queue().drain(..).collect();
//     //     let round = self.inner.current_round();
//     //     let messages = RoundMsg::from_round(round, messages);
//     //     Ok(serde_wasm_bindgen::to_value(&(round, &messages))?)
//     // }
//     //
//     // /// Create the key share.
//     // pub fn create(&mut self) -> Result<JsValue, JsError> {
//     //     let local_key = self.inner.pick_output().unwrap()?;
//     //     let key_share: KeyShare = local_key.into();
//     //     Ok(serde_wasm_bindgen::to_value(&key_share)?)
//     // }
// }
