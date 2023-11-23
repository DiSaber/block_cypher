use aes_gcm_siv::{
    aead::{generic_array::GenericArray, Aead, KeyInit},
    Aes256GcmSiv, Nonce,
};

use crate::{data_container::DataContainer, file_container::FileContainer};

pub fn from_encrypted(
    cipher_text: &Vec<u8>,
    password: &[u8; 32],
) -> Result<FileContainer, Box<dyn std::error::Error>> {
    let data_container = bincode::deserialize::<DataContainer>(&cipher_text)?;

    let cipher = Aes256GcmSiv::new(GenericArray::from_slice(password));
    let nonce = Nonce::from_slice(&data_container.nonce);

    let plaintext = match cipher.decrypt(nonce, data_container.data.as_slice()) {
        Ok(plaintext) => plaintext,
        Err(_) => Err("Failed to decrypt")?,
    };

    Ok(bincode::deserialize(&plaintext)?)
}

pub fn to_encrypted(
    file: &FileContainer,
    password: &[u8; 32],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let data = bincode::serialize(file)?;

    let cipher = Aes256GcmSiv::new(GenericArray::from_slice(password));
    let nonce_array: [u8; 12] = rand::random();
    let nonce = Nonce::from_slice(&nonce_array);

    let ciphertext = match cipher.encrypt(nonce, data.as_slice()) {
        Ok(ciphertext) => ciphertext,
        Err(_) => Err("Failed to encrypt")?,
    };

    let data_container = bincode::serialize(&DataContainer {
        data: ciphertext,
        nonce: nonce_array,
    })?;

    Ok(data_container)
}
