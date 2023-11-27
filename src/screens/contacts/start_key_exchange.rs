use std::sync::{Arc, Mutex};

use base64::{engine::general_purpose, Engine};
use fltk::{prelude::*, *};
use oqs::*;

use arboard::Clipboard;

use crate::{contact::Contact, program_data::ProgramData, screens::builders, utils::save_config};

use super::{contacts_menu, key_exchange};

pub fn start_key_exchange(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
    let mut built_start_key_exchange_menu =
        builders::build_start_key_exchange_menu(&mut main_window);

    let kyber = kem::Kem::new(kem::Algorithm::Kyber1024).unwrap();
    let (public_key, secret_key) = kyber.keypair().unwrap();
    let encoded_public_key = Arc::new(general_purpose::STANDARD_NO_PAD.encode(public_key));
    let secret_key = Arc::new(secret_key);

    built_start_key_exchange_menu.back_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| key_exchange(main_window.clone(), Arc::clone(&program_data))
    });

    built_start_key_exchange_menu
        .copy_public_key_button
        .set_callback({
            let encoded_public_key = encoded_public_key.clone();

            move |_| {
                let mut clipboard = Clipboard::new().unwrap();
                let _ = clipboard.set_text(encoded_public_key.as_str());
            }
        });

    built_start_key_exchange_menu
        .add_contact_button
        .set_callback({
            let main_window = main_window.clone();
            let contact_name_field = built_start_key_exchange_menu.contact_name_field.clone();
            let program_data = Arc::clone(&program_data);
            let secret_key = secret_key.clone();

            move |_| {
                let contact_name = contact_name_field.value().trim().to_string();

                if contact_name.is_empty() {
                    built_start_key_exchange_menu
                        .error_label
                        .set_label("Contact name cannot be empty!");
                    built_start_key_exchange_menu.error_label.show();
                    return;
                }

                {
                    let mut program_data = program_data.lock().unwrap();

                    if program_data
                        .contacts
                        .iter()
                        .any(|contact| contact.contact_name == contact_name)
                    {
                        built_start_key_exchange_menu
                            .error_label
                            .set_label("Contact already exists!");
                        built_start_key_exchange_menu.error_label.show();
                        return;
                    }

                    let cipher_text = match general_purpose::STANDARD_NO_PAD.decode(
                        built_start_key_exchange_menu
                            .cipher_text_field
                            .value()
                            .trim(),
                    ) {
                        Ok(cipher_text) => cipher_text,
                        Err(_) => {
                            built_start_key_exchange_menu
                                .error_label
                                .set_label("Invalid cipher text!");
                            built_start_key_exchange_menu.error_label.show();
                            return;
                        }
                    };

                    let cipher_text = match kyber.ciphertext_from_bytes(cipher_text.as_slice()) {
                        Some(cipher_text) => cipher_text,
                        None => {
                            built_start_key_exchange_menu
                                .error_label
                                .set_label("Invalid cipher text!");
                            built_start_key_exchange_menu.error_label.show();
                            return;
                        }
                    };

                    let shared_secret = kyber.decapsulate(&*secret_key, cipher_text).unwrap();

                    program_data.contacts.push(Contact {
                        contact_name,
                        contact_key: shared_secret.into_vec().as_slice().try_into().unwrap(),
                    });

                    save_config(&program_data, &program_data.hashed_password);
                }

                contacts_menu(main_window.clone(), Arc::clone(&program_data));
            }
        });
}
