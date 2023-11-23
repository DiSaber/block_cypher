use aes_gcm_siv::{
    aead::{generic_array::GenericArray, Aead, KeyInit},
    Aes256GcmSiv, Nonce,
};
use base64::{engine::general_purpose, Engine as _};
use serde::{Deserialize, Serialize};

use crate::data_container::DataContainer;

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

pub fn to_encrypted(
    file: &Vec<u8>,
    password: &[u8; 32],
) -> Result<String, Box<dyn std::error::Error>> {
    let data = serde_json::to_string(file)?;

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
