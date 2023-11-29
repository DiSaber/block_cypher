use std::sync::{Arc, Mutex};

use base64::{engine::general_purpose, Engine};
use fltk::{prelude::*, *};
use oqs::*;

use arboard::Clipboard;

use crate::{contact::Contact, program_data::ProgramData, screens::builders};

use super::{contacts_menu, key_exchange};

pub fn receive_key_exchange(
    mut main_window: window::Window,
    program_data: Arc<Mutex<ProgramData>>,
) {
    let mut built_receive_key_exchange_menu =
        builders::build_receive_key_exchange_menu(&mut main_window);

    built_receive_key_exchange_menu.back_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| key_exchange(main_window.clone(), Arc::clone(&program_data))
    });

    let shared_secret: Arc<Mutex<Option<[u8; 32]>>> = Arc::new(Mutex::new(None));

    built_receive_key_exchange_menu
        .copy_cipher_text_button
        .set_callback({
            let shared_secret = Arc::clone(&shared_secret);
            let receiving_key_field = built_receive_key_exchange_menu.receiving_key_field.clone();
            let mut error_label = built_receive_key_exchange_menu.error_label.clone();

            move |_| {
                let public_key = match general_purpose::STANDARD_NO_PAD
                    .decode(receiving_key_field.value().trim())
                {
                    Ok(public_key) => public_key,
                    Err(_) => {
                        error_label.set_label("Invalid receiving key!");
                        error_label.show();
                        return;
                    }
                };

                let kyber = kem::Kem::new(kem::Algorithm::Kyber1024).unwrap();
                let public_key = match kyber.public_key_from_bytes(public_key.as_slice()) {
                    Some(public_key) => public_key,
                    None => {
                        error_label.set_label("Invalid receiving key!");
                        error_label.show();
                        return;
                    }
                };

                let (cipher_text, new_shared_secret) = kyber.encapsulate(public_key).unwrap();
                let encoded_cipher_text = general_purpose::STANDARD_NO_PAD.encode(cipher_text);

                let mut clipboard = Clipboard::new().unwrap();
                let _ = clipboard.set_text(&encoded_cipher_text);

                *shared_secret.lock().unwrap() =
                    Some(new_shared_secret.into_vec().as_slice().try_into().unwrap());
            }
        });

    built_receive_key_exchange_menu
        .add_contact_button
        .set_callback({
            let main_window = main_window.clone();
            let contact_name_field = built_receive_key_exchange_menu.contact_name_field.clone();
            let program_data = Arc::clone(&program_data);
            let mut error_label = built_receive_key_exchange_menu.error_label.clone();

            move |_| {
                let contact_name = contact_name_field.value().trim().to_string();

                if contact_name.is_empty() {
                    error_label.set_label("Contact name cannot be empty!");
                    error_label.show();
                    return;
                }

                {
                    let mut program_data = program_data.lock().unwrap();

                    if program_data
                        .contacts
                        .iter()
                        .any(|contact| contact.contact_name == contact_name)
                    {
                        error_label.set_label("Contact already exists!");
                        error_label.show();
                        return;
                    }

                    let shared_secret = match *shared_secret.lock().unwrap() {
                        Some(shared_secret) => shared_secret,
                        None => {
                            error_label.set_label("You must copy the cipher text!");
                            error_label.show();
                            return;
                        }
                    };

                    program_data.contacts.push(Contact {
                        contact_name,
                        contact_key: shared_secret,
                    });

                    program_data.save_config();
                }

                contacts_menu(main_window.clone(), Arc::clone(&program_data));
            }
        });
}
