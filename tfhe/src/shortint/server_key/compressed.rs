//! Module with the definition of the CompressedServerKey.

use super::MaxDegree;
use crate::core_crypto::prelude::*;
use crate::shortint::engine::ShortintEngine;
use crate::shortint::parameters::{CarryModulus, MessageModulus};
use crate::shortint::ClientKey;
use serde::{Deserialize, Deserializer, Serialize, Serializer};

/// A structure containing a compressed server public key.
///
/// The server key is generated by the client and is meant to be published: the client
/// sends it to the server so it can compute homomorphic circuits.
#[derive(Clone, Debug, PartialEq)]
pub struct CompressedServerKey {
    pub key_switching_key: SeededLweKeyswitchKey64,
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
    pub fn new(client_key: &ClientKey) -> CompressedServerKey {
        ShortintEngine::with_thread_local_mut(|engine| {
            engine.new_compressed_server_key(client_key).unwrap()
        })
    }
}

#[derive(Serialize, Deserialize)]
pub(super) struct SerializableCompressedServerKey {
    pub key_switching_key: Vec<u8>,
    pub bootstrapping_key: Vec<u8>,
    // Size of the message buffer
    pub message_modulus: MessageModulus,
    // Size of the carry buffer
    pub carry_modulus: CarryModulus,
    // Maximum number of operations that can be done before emptying the operation buffer
    pub max_degree: MaxDegree,
}

impl Serialize for CompressedServerKey {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let key_switching_key =
            bincode::serialize(&self.key_switching_key).map_err(serde::ser::Error::custom)?;
        let bootstrapping_key =
            bincode::serialize(&self.bootstrapping_key).map_err(serde::ser::Error::custom)?;

        SerializableCompressedServerKey {
            key_switching_key,
            bootstrapping_key,
            message_modulus: self.message_modulus,
            carry_modulus: self.carry_modulus,
            max_degree: self.max_degree,
        }
        .serialize(serializer)
    }
}

impl<'de> Deserialize<'de> for CompressedServerKey {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let thing = SerializableCompressedServerKey::deserialize(deserializer)
            .map_err(serde::de::Error::custom)?;

        let key_switching_key = bincode::deserialize(thing.key_switching_key.as_slice())
            .map_err(serde::de::Error::custom)?;

        let bootstrapping_key = bincode::deserialize(thing.bootstrapping_key.as_slice())
            .map_err(serde::de::Error::custom)?;

        Ok(Self {
            key_switching_key,
            bootstrapping_key,
            message_modulus: thing.message_modulus,
            carry_modulus: thing.carry_modulus,
            max_degree: thing.max_degree,
        })
    }
}
