//! Module with the definition of the CompressedServerKey.

use super::MaxDegree;
use crate::core_crypto::prelude::*;
use crate::shortint::engine::ShortintEngine;
use crate::shortint::parameters::{CarryModulus, MessageModulus};
use crate::shortint::{ClientKey, PBSOrderMarker};
use serde::{Deserialize, Serialize};

/// A structure containing a compressed server public key.
///
/// The server key is generated by the client and is meant to be published: the client
/// sends it to the server so it can compute homomorphic circuits.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct CompressedServerKey {
    pub key_switching_key: SeededLweKeyswitchKeyOwned<u64>,
    pub bootstrapping_key: SeededLweBootstrapKeyOwned<u64>,
    // Size of the message buffer
    pub message_modulus: MessageModulus,
    // Size of the carry buffer
    pub carry_modulus: CarryModulus,
    // Maximum number of operations that can be done before emptying the operation buffer
    pub max_degree: MaxDegree,
}

impl CompressedServerKey {
    /// Generate a compressed server key.
    ///
    /// # Example
    ///
    /// ```rust
    /// use tfhe::shortint::client_key::ClientKey;
    /// use tfhe::shortint::parameters::Parameters;
    /// use tfhe::shortint::server_key::CompressedServerKey;
    ///
    /// // Generate the client key:
    /// let cks = ClientKey::new(Parameters::default());
    ///
    /// let sks = CompressedServerKey::new(&cks);
    /// ```
    pub fn new<OpOrder: PBSOrderMarker>(client_key: &ClientKey<OpOrder>) -> CompressedServerKey {
        ShortintEngine::with_thread_local_mut(|engine| {
            engine.new_compressed_server_key(client_key).unwrap()
        })
    }
}
