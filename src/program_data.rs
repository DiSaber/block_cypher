use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

use crate::encryption_handler::RECOMMENDED_HASH_ITERATIONS;

#[derive(Serialize, Deserialize)]
pub struct ProgramData {
    pub hashed_password: [u8; 32],
}

impl ProgramData {
    pub fn new(password: &String) -> Self {
        let mut hashed_password = Sha3_256::digest(password);

        for _ in 0..(RECOMMENDED_HASH_ITERATIONS - 1) {
            hashed_password = Sha3_256::digest(hashed_password);
        }

        ProgramData {
            hashed_password: hashed_password.as_slice().try_into().unwrap(),
        }
    }
}
