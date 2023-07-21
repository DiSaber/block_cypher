use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct DataContainer {
    pub data: Vec<u8>,
    pub nonce: [u8; 12],
}
