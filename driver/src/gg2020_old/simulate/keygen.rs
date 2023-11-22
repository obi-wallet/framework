//! Key generation simulated

use cggmp_threshold_ecdsa::mpc_ecdsa::gg_2020::state_machine::sign::Error;
use round_based::dev::AsyncSimulation;

use mpc_protocol::Parameters;

use crate::gg20::KeyShare;
use crate::gg_2020::state_machine::keygen::Keygen;

/// Generate keys in simulation mode
pub async fn keygen_simulated_impl(
    parameters: Parameters,
) -> Result<Vec<KeyShare>, Error> {
    let t = parameters.threshold;
    let n = parameters.parties;
    let mut simulation = AsyncSimulation::<Keygen>::new();

    for i in 1..=n {
        simulation.add_party(Keygen::new(i, t, n).unwrap());
    }

    let keys: Vec<KeyShare> = simulation
        .run()
        .await
        .into_iter()
        .map(|k| k.unwrap())
        .collect();

    Ok(keys.into_iter().map(|k| k.into()).collect())
}
