mod data_container;
mod encryption_handler;
mod program_data;

use encryption_handler::{from_encrypted, to_encrypted};
use oqs::*;
use program_data::ProgramData;
use std::io::{self, Write};

fn main() {
    let program_data = ProgramData::new(&String::from("hello"));
    let encrypted = to_encrypted(&program_data, &program_data.hashed_password).unwrap();
    println!("{}", encrypted);
    let decrypted: ProgramData = from_encrypted(&encrypted, &program_data.hashed_password).unwrap();
    println!(
        "{}",
        program_data.hashed_password == decrypted.hashed_password
    );
}

/*
let mut buffer = String::new();
    print!("s or r: ");

    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();

    if buffer.trim_end() == "s" {
        let kyber = kem::Kem::new(kem::Algorithm::Kyber1024).unwrap();
        let (public_key, secret_key) = kyber.keypair().unwrap();

        println!("{}", general_purpose::STANDARD_NO_PAD.encode(public_key));

        let mut buffer = String::new();
        print!("Cipher Text: ");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();

        let cipher_text = general_purpose::STANDARD_NO_PAD
            .decode(buffer.trim_end())
            .unwrap();

        let cipher_text = kyber.ciphertext_from_bytes(cipher_text.as_slice()).unwrap();
        let shared_secret = kyber.decapsulate(&secret_key, &cipher_text).unwrap();

        println!(
            "Shared Secret: {}",
            general_purpose::STANDARD_NO_PAD.encode(shared_secret)
        );
    } else {
        let kyber = kem::Kem::new(kem::Algorithm::Kyber1024).unwrap();

        let mut buffer = String::new();
        print!("Public Key: ");

        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer).unwrap();

        let public_key = general_purpose::STANDARD_NO_PAD
            .decode(buffer.trim_end())
            .unwrap();
        let public_key = kyber.public_key_from_bytes(public_key.as_slice()).unwrap();

        let (cipher_text, shared_secret) = kyber.encapsulate(public_key).unwrap();
        println!(
            "Cipher Text: {}\n\nShared Secret: {}",
            general_purpose::STANDARD_NO_PAD.encode(cipher_text),
            general_purpose::STANDARD_NO_PAD.encode(shared_secret)
        );
    }
 */
