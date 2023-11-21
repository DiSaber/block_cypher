use std::sync::{Arc, Mutex};

use base64::{engine::general_purpose, Engine};
use fltk::{enums::Color, prelude::*, *};
use oqs::*;
use sha3::{Digest, Sha3_256};

use arboard::Clipboard;

use crate::{contact::Contact, program_data::ProgramData, screens, utils::save_config};

pub fn contacts(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
    main_window.clear();
    main_window.begin();

    let mut back_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(20, 20)
        .with_label("Back");
    back_button.set_color(Color::from_hex(0x545454));
    back_button.set_label_color(Color::White);
    back_button.set_label_size(16);

    let mut view_key = frame::Frame::default()
        .with_size(150, 60)
        .with_pos(150, 127);
    view_key.set_label_color(Color::White);
    view_key.set_label_size(14);

    let program_data_unlocked = program_data.lock().unwrap();

    let mut contacts_dropdown = menu::Choice::default()
        .with_size(120, 30)
        .with_pos(340, 142);
    contacts_dropdown.add_choice(&program_data_unlocked.format_contacts());

    let mut contacts_count = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 172)
        .with_label(&format!(
            "{} {}",
            program_data_unlocked.contacts.len(),
            if program_data_unlocked.contacts.len() == 1 {
                "contact"
            } else {
                "contacts"
            }
        ));
    contacts_count.set_label_color(Color::White);
    contacts_count.set_label_size(14);

    let mut edit_contact_button = button::Button::default()
        .with_size(100, 20)
        .with_pos(490, 147)
        .with_label("Edit Contact");
    edit_contact_button.set_color(Color::from_hex(0x545454));
    edit_contact_button.set_label_color(Color::White);
    edit_contact_button.set_label_size(12);

    let mut add_contact_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 290)
        .with_label("Add Contact");
    add_contact_button.set_color(Color::from_hex(0x545454));
    add_contact_button.set_label_color(Color::White);
    add_contact_button.set_label_size(16);

    main_window.end();
    main_window.redraw();

    back_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::main_menu::main_menu(main_window.clone(), Arc::clone(&program_data))
    });

    contacts_dropdown.set_callback({
        let program_data = Arc::clone(&program_data);
        let contacts_dropdown = contacts_dropdown.clone();
        let mut view_key = view_key.clone();

        move |_| {
            if let Some(contact_name) = contacts_dropdown.choice() {
                let program_data_unlocked = program_data.lock().unwrap();
                let contact = program_data_unlocked
                    .contacts
                    .iter()
                    .find(|contact| contact.contact_name == contact_name)
                    .unwrap();

                let digest = Sha3_256::digest(contact.contact_key);
                let hex_digest = hex::encode(digest.as_slice());

                view_key.set_label(
                    &hex_digest
                        .chars()
                        .enumerate()
                        .flat_map(|(i, c)| {
                            if i != 0 && i % 4 == 0 {
                                Some(if i % 16 == 0 { '\n' } else { ' ' })
                            } else {
                                None
                            }
                            .into_iter()
                            .chain(std::iter::once(c))
                        })
                        .collect::<String>(),
                );
            }
        }
    });

    edit_contact_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);
        let contacts_dropdown = contacts_dropdown.clone();

        move |_| {
            if let Some(contact_name) = contacts_dropdown.choice() {
                edit_contact(
                    main_window.clone(),
                    Arc::clone(&program_data),
                    &contact_name,
                )
            }
        }
    });

    add_contact_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| key_exchange_menu(main_window.clone(), Arc::clone(&program_data))
    });
}

fn edit_contact(
    mut main_window: window::Window,
    program_data: Arc<Mutex<ProgramData>>,
    contact_name: &str,
) {
    let program_data_unlocked = program_data.lock().unwrap();
    let contact_index = program_data_unlocked
        .contacts
        .iter()
        .position(|contact| contact.contact_name == contact_name)
        .unwrap();

    main_window.clear();
    main_window.begin();

    let mut back_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(20, 20)
        .with_label("Back");
    back_button.set_color(Color::from_hex(0x545454));
    back_button.set_label_color(Color::White);
    back_button.set_label_size(16);

    let mut contact_name_field = input::Input::default()
        .with_size(200, 24)
        .with_pos(300, 150);
    contact_name_field.set_color(Color::from_hex(0x545454));
    contact_name_field.set_text_color(Color::White);
    contact_name_field.set_text_size(16);
    contact_name_field.set_value(contact_name);

    let mut contact_name_text = frame::Frame::default()
        .with_size(300, 24)
        .with_pos(100, 150)
        .with_label("Contact Name: ");
    contact_name_text.set_label_color(Color::White);
    contact_name_text.set_label_size(14);

    let mut save_contact_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 200)
        .with_label("Save Contact");
    save_contact_button.set_color(Color::from_hex(0x545454));
    save_contact_button.set_label_color(Color::White);
    save_contact_button.set_label_size(16);

    let mut error_label = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 235);
    error_label.set_label_color(Color::from_hex(0xFF3D3D));
    error_label.set_label_size(14);
    error_label.hide();

    let mut delete_contact_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 300)
        .with_label("Delete Contact");
    delete_contact_button.set_color(Color::from_hex(0x545454));
    delete_contact_button.set_label_color(Color::White);
    delete_contact_button.set_label_size(16);

    main_window.end();
    main_window.redraw();

    back_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| contacts(main_window.clone(), Arc::clone(&program_data))
    });

    save_contact_button.set_callback({
        let main_window = main_window.clone();
        let contact_name_field = contact_name_field.clone();
        let mut error_label = error_label.clone();
        let program_data = Arc::clone(&program_data);
        let contact_name = contact_name.to_string();

        move |_| {
            let contact_name_from_field = contact_name_field.value();
            let contact_name_from_field = contact_name_from_field.trim();

            if contact_name_from_field.is_empty() {
                error_label.set_label("Contact name cannot be empty!");
                error_label.show();
                return;
            }

            {
                let mut program_data_unlocked = program_data.lock().unwrap();

                if program_data_unlocked.contacts.iter().any(|contact| {
                    contact.contact_name == contact_name_from_field
                        && contact.contact_name != contact_name
                }) {
                    error_label.set_label("Contact already exists!");
                    error_label.show();
                    return;
                }

                program_data_unlocked.contacts[contact_index].contact_name =
                    contact_name_from_field.to_string();

                save_config(
                    &program_data_unlocked,
                    &program_data_unlocked.hashed_password,
                );
            }

            contacts(main_window.clone(), Arc::clone(&program_data));
        }
    });

    delete_contact_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| {
            {
                let mut program_data_unlocked = program_data.lock().unwrap();

                program_data_unlocked.contacts.remove(contact_index);

                save_config(
                    &program_data_unlocked,
                    &program_data_unlocked.hashed_password,
                );
            }

            contacts(main_window.clone(), Arc::clone(&program_data));
        }
    });
}

fn key_exchange_menu(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
    main_window.clear();
    main_window.begin();

    let mut back_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(20, 20)
        .with_label("Back");
    back_button.set_color(Color::from_hex(0x545454));
    back_button.set_label_color(Color::White);
    back_button.set_label_size(16);

    let mut start_key_exchange_button = button::Button::default()
        .with_size(175, 30)
        .with_pos(312, 207)
        .with_label("Start Key Exchange");
    start_key_exchange_button.set_color(Color::from_hex(0x545454));
    start_key_exchange_button.set_label_color(Color::White);
    start_key_exchange_button.set_label_size(16);

    let mut receive_key_button = button::Button::default()
        .with_size(175, 30)
        .with_pos(312, 257)
        .with_label("Enter a Receiving Key");
    receive_key_button.set_color(Color::from_hex(0x545454));
    receive_key_button.set_label_color(Color::White);
    receive_key_button.set_label_size(16);

    main_window.end();
    main_window.redraw();

    back_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| contacts(main_window.clone(), Arc::clone(&program_data))
    });

    start_key_exchange_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| start_key_exchange(main_window.clone(), Arc::clone(&program_data))
    });

    receive_key_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| recieve_key_exchange(main_window.clone(), Arc::clone(&program_data))
    });
}

fn recieve_key_exchange(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
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

    let mut recieving_key_text = frame::Frame::default()
        .with_size(300, 24)
        .with_pos(25, 190)
        .with_label("Recieving Key: ");
    recieving_key_text.set_label_color(Color::White);
    recieving_key_text.set_label_size(14);

    let mut recieving_key_field = input::Input::default()
        .with_size(350, 24)
        .with_pos(225, 190);
    recieving_key_field.set_color(Color::from_hex(0x545454));
    recieving_key_field.set_text_color(Color::White);
    recieving_key_field.set_text_size(16);

    let mut copy_cipher_text_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 300)
        .with_label("Copy Cipher Text");
    copy_cipher_text_button.set_color(Color::from_hex(0x545454));
    copy_cipher_text_button.set_label_color(Color::White);
    copy_cipher_text_button.set_label_size(16);

    let mut description_text = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 230)
        .with_label("Enter the receiving key above and send the cipher text back");
    description_text.set_label_color(Color::White);
    description_text.set_label_size(14);

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

        move |_| key_exchange_menu(main_window.clone(), Arc::clone(&program_data))
    });

    let shared_secret: Arc<Mutex<Option<[u8; 32]>>> = Arc::new(Mutex::new(None));

    copy_cipher_text_button.set_callback({
        let shared_secret = Arc::clone(&shared_secret);
        let recieving_key_field = recieving_key_field.clone();
        let mut error_label = error_label.clone();

        move |_| {
            let public_key =
                match general_purpose::STANDARD_NO_PAD.decode(recieving_key_field.value().trim()) {
                    Ok(public_key) => public_key,
                    Err(_) => {
                        error_label.set_label("Invalid recieving key!");
                        error_label.show();
                        return;
                    }
                };

            let kyber = kem::Kem::new(kem::Algorithm::Kyber1024).unwrap();
            let public_key = match kyber.public_key_from_bytes(public_key.as_slice()) {
                Some(public_key) => public_key,
                None => {
                    error_label.set_label("Invalid recieving key!");
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

    add_contact_button.set_callback({
        let main_window = main_window.clone();
        let contact_name_field = contact_name_field.clone();
        let program_data = Arc::clone(&program_data);
        let mut error_label = error_label.clone();

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

                let shared_secret = match *shared_secret.lock().unwrap() {
                    Some(shared_secret) => shared_secret,
                    None => {
                        error_label.set_label("You must copy the cipher text!");
                        error_label.show();
                        return;
                    }
                };

                program_data_unlocked.contacts.push(Contact {
                    contact_name: contact_name.to_string(),
                    contact_key: shared_secret,
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

fn start_key_exchange(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
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

        move |_| key_exchange_menu(main_window.clone(), Arc::clone(&program_data))
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
