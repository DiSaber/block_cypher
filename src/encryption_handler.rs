use aes_gcm_siv::{
    aead::{generic_array::GenericArray, Aead, KeyInit},
    Aes256GcmSiv, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use rand;
use serde::{Deserialize, Serialize};
use sha3::{Digest, Sha3_256};

use crate::data_container::DataContainer;

const RECOMMENDED_HASH_ITERATIONS: i32 = 10000;

pub fn from_encrypted<T>(cipher_text: &str, password: &[u8; 32]) -> Result<T, String>
where
    T: for<'a> Deserialize<'a>,
{
    let data_container = match general_purpose::STANDARD_NO_PAD.decode(cipher_text.trim_end()) {
        Ok(data_container) => data_container,
        Err(_) => return Err(String::from("Failed to decode the base64 text")),
    };

    let data_container = match serde_json::from_slice::<DataContainer>(&data_container) {
        Ok(data_container) => data_container,
        Err(_) => return Err(String::from("Failed to decode data container json")),
    };

    let cipher = Aes256GcmSiv::new(GenericArray::from_slice(password));
    let nonce = Nonce::from_slice(&data_container.nonce);

    let plaintext = match cipher.decrypt(nonce, data_container.data.as_slice()) {
        Ok(plaintext) => plaintext,
        Err(_) => return Err(String::from("Failed to decrypt the cipher text")),
    };

    match serde_json::from_slice::<T>(&plaintext) {
        Ok(data) => Ok(data),
        Err(_) => Err(String::from("Failed to decode the data json")),
    }
}

pub fn to_encrypted<T>(data: &T, password: &[u8; 32]) -> Result<String, String>
where
    T: Serialize,
{
    let data = match serde_json::to_string::<T>(data) {
        Ok(data) => data,
        Err(_) => return Err(String::from("Failed to encode the data json")),
    };

    let cipher = Aes256GcmSiv::new(GenericArray::from_slice(password));
    let nonce_array: [u8; 12] = rand::random();
    let nonce = Nonce::from_slice(&nonce_array);

    let ciphertext = match cipher.encrypt(nonce, data.as_bytes()) {
        Ok(ciphertext) => ciphertext,
        Err(_) => return Err(String::from("Failed to encrypt the data json")),
    };

    let data_container = match serde_json::to_string::<DataContainer>(&DataContainer {
        data: ciphertext,
        nonce: nonce_array,
    }) {
        Ok(data_container) => data_container,
        Err(_) => return Err(String::from("Failed to encode the data container json")),
    };

    Ok(general_purpose::STANDARD_NO_PAD.encode(data_container))
}

pub fn hash_password(password: &str) -> [u8; 32] {
    let mut hashed_password = Sha3_256::digest(password);

    for _ in 0..(RECOMMENDED_HASH_ITERATIONS - 1) {
        hashed_password = Sha3_256::digest(hashed_password);
    }

    hashed_password.as_slice().try_into().unwrap()
}
