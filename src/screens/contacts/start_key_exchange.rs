use std::sync::{Arc, Mutex};

use base64::{engine::general_purpose, Engine};
use fltk::{enums::Color, prelude::*, *};
use oqs::*;

use arboard::Clipboard;

use crate::{contact::Contact, program_data::ProgramData, utils::save_config};

use super::{contacts, key_exchange};

pub fn start_key_exchange(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
    let kyber = kem::Kem::new(kem::Algorithm::Kyber1024).unwrap();
    let (public_key, secret_key) = kyber.keypair().unwrap();
    let encoded_public_key = Arc::new(general_purpose::STANDARD_NO_PAD.encode(public_key));
    let secret_key = Arc::new(secret_key);

    main_window.clear();
    main_window.begin();

    let mut back_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(20, 20)
        .with_label("Back");
    back_button.set_color(Color::from_hex(0x545454));
    back_button.set_label_color(Color::White);
    back_button.set_label_size(16);

    let mut contact_name_text = frame::Frame::default()
        .with_size(300, 24)
        .with_pos(100, 150)
        .with_label("Contact Name: ");
    contact_name_text.set_label_color(Color::White);
    contact_name_text.set_label_size(14);

    let mut contact_name_field = input::Input::default()
        .with_size(200, 24)
        .with_pos(300, 150);
    contact_name_field.set_color(Color::from_hex(0x545454));
    contact_name_field.set_text_color(Color::White);
    contact_name_field.set_text_size(16);

    let mut copy_public_key_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 190)
        .with_label("Copy Receiving Key");
    copy_public_key_button.set_color(Color::from_hex(0x545454));
    copy_public_key_button.set_label_color(Color::White);
    copy_public_key_button.set_label_size(16);

    let mut description_text = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 230)
        .with_label("Send this receiving key to the other user and enter the cipher text they send back to you");
    description_text.set_label_color(Color::White);
    description_text.set_label_size(14);

    let mut cipher_text = frame::Frame::default()
        .with_size(300, 24)
        .with_pos(30, 300)
        .with_label("Cipher Text: ");
    cipher_text.set_label_color(Color::White);
    cipher_text.set_label_size(14);

    let mut cipher_text_field = input::Input::default()
        .with_size(350, 24)
        .with_pos(225, 300);
    cipher_text_field.set_color(Color::from_hex(0x545454));
    cipher_text_field.set_text_color(Color::White);
    cipher_text_field.set_text_size(16);

    let mut add_contact_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 350)
        .with_label("Add Contact");
    add_contact_button.set_color(Color::from_hex(0x545454));
    add_contact_button.set_label_color(Color::White);
    add_contact_button.set_label_size(16);

    let mut error_label = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 385);
    error_label.set_label_color(Color::from_hex(0xFF3D3D));
    error_label.set_label_size(14);
    error_label.hide();

    main_window.end();
    main_window.redraw();

    back_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| key_exchange(main_window.clone(), Arc::clone(&program_data))
    });

    copy_public_key_button.set_callback({
        let encoded_public_key = encoded_public_key.clone();

        move |_| {
            let mut clipboard = Clipboard::new().unwrap();
            let _ = clipboard.set_text(encoded_public_key.as_str());
        }
    });

    add_contact_button.set_callback({
        let main_window = main_window.clone();
        let contact_name_field = contact_name_field.clone();
        let program_data = Arc::clone(&program_data);
        let secret_key = secret_key.clone();

        move |_| {
            let contact_name = contact_name_field.value();
            let contact_name = contact_name.trim();

            if contact_name.is_empty() {
                error_label.set_label("Contact name cannot be empty!");
                error_label.show();
                return;
            }

            {
                let mut program_data_unlocked = program_data.lock().unwrap();

                if program_data_unlocked
                    .contacts
                    .iter()
                    .any(|contact| contact.contact_name == contact_name)
                {
                    error_label.set_label("Contact already exists!");
                    error_label.show();
                    return;
                }

                let cipher_text = match general_purpose::STANDARD_NO_PAD
                    .decode(cipher_text_field.value().trim().trim_end())
                {
                    Ok(cipher_text) => cipher_text,
                    Err(_) => {
                        error_label.set_label("Invalid cipher text!");
                        error_label.show();
                        return;
                    }
                };

                let cipher_text = match kyber.ciphertext_from_bytes(cipher_text.as_slice()) {
                    Some(cipher_text) => cipher_text,
                    None => {
                        error_label.set_label("Invalid cipher text!");
                        error_label.show();
                        return;
                    }
                };

                let shared_secret = kyber.decapsulate(&*secret_key, cipher_text).unwrap();

                program_data_unlocked.contacts.push(Contact {
                    contact_name: contact_name.to_string(),
                    contact_key: shared_secret.into_vec().as_slice().try_into().unwrap(),
                });

                save_config(
                    &program_data_unlocked,
                    &program_data_unlocked.hashed_password,
                );
            }

            contacts(main_window.clone(), Arc::clone(&program_data));
        }
    });
}
