use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Contact {
    pub contact_name: String,
    pub contact_key: [u8; 32],
}
