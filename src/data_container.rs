use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DataContainer {
    pub data: Vec<u8>,
    pub nonce: [u8; 12],
}

impl DataContainer {
    pub fn to_binary(&self) -> Vec<u8> {
        bincode::serialize(self).unwrap()
    }

    pub fn from_binary(data: &[u8]) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(bincode::deserialize::<Self>(data)?)
    }
}
