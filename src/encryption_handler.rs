use aes_gcm_siv::{
    aead::{generic_array::GenericArray, Aead, KeyInit},
    Aes256GcmSiv, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

use crate::data_container::DataContainer;

const RECOMMENDED_HASH_ITERATIONS: i32 = 100000;

pub fn from_encrypted<T>(
    cipher_text: &str,
    password: &[u8; 32],
) -> Result<T, Box<dyn std::error::Error>>
where
    T: for<'a> Deserialize<'a>,
{
    let data_container = general_purpose::STANDARD_NO_PAD.decode(cipher_text.trim_end())?;

    let data_container = serde_json::from_slice::<DataContainer>(&data_container)?;

    let cipher = Aes256GcmSiv::new(GenericArray::from_slice(password));
    let nonce = Nonce::from_slice(&data_container.nonce);

    let plaintext = match cipher.decrypt(nonce, data_container.data.as_slice()) {
        Ok(plaintext) => plaintext,
        Err(_) => Err("Failed to decrypt")?,
    };

    Ok(serde_json::from_slice(&plaintext)?)
}

pub fn to_encrypted<T>(data: &T, password: &[u8; 32]) -> Result<String, Box<dyn std::error::Error>>
where
    T: Serialize,
{
    let data = serde_json::to_string(data)?;

    let cipher = Aes256GcmSiv::new(GenericArray::from_slice(password));
    let nonce_array: [u8; 12] = rand::random();
    let nonce = Nonce::from_slice(&nonce_array);

    let ciphertext = match cipher.encrypt(nonce, data.as_bytes()) {
        Ok(ciphertext) => ciphertext,
        Err(_) => Err("Failed to encrypt")?,
    };

    let data_container = serde_json::to_string(&DataContainer {
        data: ciphertext,
        nonce: nonce_array,
    })?;

    Ok(general_purpose::STANDARD_NO_PAD.encode(data_container))
}

pub fn hash_password(password: &str) -> [u8; 32] {
    let mut hashed_password = Sha3_256::digest(password);

    for _ in 0..(RECOMMENDED_HASH_ITERATIONS - 1) {
        hashed_password = Sha3_256::digest(hashed_password);
    }

    hashed_password.as_slice().try_into().unwrap()
}
