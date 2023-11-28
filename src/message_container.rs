use base64::{engine::general_purpose, Engine};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

use crate::data_container::DataContainer;

#[derive(Serialize, Deserialize)]
pub struct MessageContainer {
    pub data_container: DataContainer,
    pub message_nonce: [u8; 32],
    pub user_id: [u8; 32],
}

impl MessageContainer {
    pub fn new(data_container: DataContainer, password: &[u8; 32]) -> Self {
        let message_nonce = rand::random();
        let user_id = Sha3_256::digest([*password, message_nonce].concat())
            .as_slice()
            .try_into()
            .unwrap();
        MessageContainer {
            data_container,
            message_nonce,
            user_id,
        }
    }

    pub fn validate_user_id(&self, password: &[u8; 32]) -> bool {
        let user_id: [u8; 32] = Sha3_256::digest([*password, self.message_nonce].concat())
            .as_slice()
            .try_into()
            .unwrap();
        self.user_id == user_id
    }

    pub fn to_base64(&self) -> String {
        general_purpose::STANDARD_NO_PAD.encode(self.to_binary())
    }

    pub fn from_base64(data: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let data_container = general_purpose::STANDARD_NO_PAD.decode(data.trim())?;
        Self::from_binary(&data_container)
    }

    pub fn to_binary(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn from_binary(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(bincode::deserialize::<Self>(data)?)
    }
}
