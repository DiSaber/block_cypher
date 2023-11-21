use std::sync::{Arc, Mutex};

use base64::{engine::general_purpose, Engine};
use fltk::{enums::Color, prelude::*, *};
use oqs::*;

use arboard::Clipboard;

use crate::{contact::Contact, program_data::ProgramData, utils::save_config};

use super::{contacts, key_exchange};

pub fn recieve_key_exchange(
    mut main_window: window::Window,
    program_data: Arc<Mutex<ProgramData>>,
) {
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

        move |_| key_exchange(main_window.clone(), Arc::clone(&program_data))
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
