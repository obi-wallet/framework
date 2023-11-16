//! Webassembly bindings for the gg2020 key generator.
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod bindings {
    use wasm_bindgen::{JsError, JsValue};
    use wasm_bindgen::prelude::wasm_bindgen;

    use mpc_driver::gg2020_old::keygen::new_keygen;
    use mpc_driver::gg_2020::state_machine::keygen::Keygen;

    #[wasm_bindgen]
    pub struct KeyGenerator(Keygen);

    #[wasm_bindgen]
    impl KeyGenerator {
        /// Create a key generator.
        #[wasm_bindgen(constructor)]
        pub fn new(
            parameters: JsValue,
            party_signup: JsValue,
        ) -> Result<KeyGenerator, JsError> {
            let keygen = new_keygen(
                serde_wasm_bindgen::from_value(parameters)?,
                serde_wasm_bindgen::from_value(party_signup)?,
            )?;
            Ok(KeyGenerator(keygen))
        }
        //
        // /// Handle an incoming message.
        // #[wasm_bindgen(js_name = "handleIncoming")]
        // pub fn handle_incoming(
        //     &mut self,
        //     message: JsValue,
        // ) -> Result<(), JsError> {
        //     let message: Msg<<Keygen as StateMachine>::MessageBody> =
        //         serde_wasm_bindgen::from_value(message)?;
        //     self.inner.handle_incoming(message)?;
        //     Ok(())
        // }
        //
        // /// Proceed to the next round.
        // pub fn proceed(&mut self) -> Result<JsValue, JsError> {
        //     self.inner.proceed()?;
        //     let messages = self.inner.message_queue().drain(..).collect();
        //     let round = self.inner.current_round();
        //     let messages = RoundMsg::from_round(round, messages);
        //     Ok(serde_wasm_bindgen::to_value(&(round, &messages))?)
        // }
        //
        // /// Create the key share.
        // pub fn create(&mut self) -> Result<JsValue, JsError> {
        //     let local_key = self.inner.pick_output().unwrap()?;
        //     let key_share: KeyShare = local_key.into();
        //     Ok(serde_wasm_bindgen::to_value(&key_share)?)
        // }
    }
}