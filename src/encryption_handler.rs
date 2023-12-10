use aes_gcm_siv::{
    aead::{generic_array::GenericArray, Aead, KeyInit},
    Aes256GcmSiv, Nonce,
};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

use crate::data_container::DataContainer;

const RECOMMENDED_HASH_ITERATIONS: i32 = 100_000;

pub fn from_encrypted<T>(
    data_container: &DataContainer,
    password: &[u8; 32],
) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'a> Deserialize<'a>,
{
    let cipher = Aes256GcmSiv::new(GenericArray::from_slice(password));
    let nonce = Nonce::from_slice(&data_container.nonce);

    let Ok(plaintext) = cipher.decrypt(nonce, data_container.data.as_slice()) else {
        Err("Failed to decrypt")?
    };

    Ok(bincode::deserialize(&plaintext)?)
}

pub fn to_encrypted<T>(
    data: &T,
    password: &[u8; 32],
) -> Result<DataContainer, Box<dyn std::error::Error>>
where
    T: Serialize,
{
    let data = bincode::serialize(data)?;

    let cipher = Aes256GcmSiv::new(GenericArray::from_slice(password));
    let nonce_array: [u8; 12] = rand::random();
    let nonce = Nonce::from_slice(&nonce_array);

    let Ok(ciphertext) = cipher.encrypt(nonce, data.as_slice()) else {
        Err("Failed to encrypt")?
    };

    Ok(DataContainer {
        data: ciphertext,
        nonce: nonce_array,
    })
}

pub fn hash_password(password: &str) -> [u8; 32] {
    let mut hashed_password = Sha3_256::digest(password);

    for _ in 0..(RECOMMENDED_HASH_ITERATIONS - 1) {
        hashed_password = Sha3_256::digest(hashed_password);
    }

    hashed_password.as_slice().try_into().unwrap()
}
