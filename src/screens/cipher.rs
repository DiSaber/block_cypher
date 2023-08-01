use std::sync::{Arc, Mutex};

use arboard::Clipboard;
use fltk::{enums::Color, prelude::*, *};

use crate::{
    encryption_handler::{from_encrypted, to_encrypted},
    program_data::ProgramData,
    screens,
};

pub fn encrypt(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
    main_window.clear();
    main_window.begin();

    let mut back_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(20, 20)
        .with_label("Back");
    back_button.set_color(Color::from_hex(0x545454));
    back_button.set_label_color(Color::White);
    back_button.set_label_size(16);

    let mut text_field = input::MultilineInput::default()
        .with_size(500, 240)
        .with_pos(150, 60);
    text_field.set_color(Color::from_hex(0x545454));
    text_field.set_text_color(Color::White);
    text_field.set_text_size(16);

    let program_data_unlocked = program_data.lock().unwrap();

    let mut contacts_dropdown = menu::Choice::default()
        .with_size(120, 30)
        .with_pos(340, 320);
    contacts_dropdown.add_choice(&program_data_unlocked.format_contacts());

    let mut contacts_count = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 350)
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

    let mut encrypt_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 400)
        .with_label("Copy encrypted text");
    encrypt_button.set_color(Color::from_hex(0x545454));
    encrypt_button.set_label_color(Color::White);
    encrypt_button.set_label_size(16);

    main_window.end();
    main_window.redraw();

    back_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::menu::main_menu(main_window.clone(), Arc::clone(&program_data))
    });

    encrypt_button.set_callback({
        let program_data = Arc::clone(&program_data);

        move |_| {
            if let Some(contact_name) = contacts_dropdown.choice() {
                let program_data_unlocked = program_data.lock().unwrap();
                let contact_index = program_data_unlocked
                    .contacts
                    .iter()
                    .position(|contact| contact.contact_name == contact_name)
                    .unwrap();

                let message = to_encrypted(
                    &text_field.value().trim().to_owned(),
                    &program_data_unlocked.contacts[contact_index].contact_key,
                )
                .unwrap();

                let mut clipboard = Clipboard::new().unwrap();
                let _ = clipboard.set_text(&message);
            }
        }
    });
}

pub fn decrypt(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
    main_window.clear();
    main_window.begin();

    let mut back_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(20, 20)
        .with_label("Back");
    back_button.set_color(Color::from_hex(0x545454));
    back_button.set_label_color(Color::White);
    back_button.set_label_size(16);

    let mut encrypted_text_field = input::Input::default().with_size(300, 24).with_pos(250, 60);
    encrypted_text_field.set_color(Color::from_hex(0x545454));
    encrypted_text_field.set_text_color(Color::White);
    encrypted_text_field.set_text_size(16);

    let mut encrypted_text = frame::Frame::default()
        .with_size(300, 24)
        .with_pos(50, 60)
        .with_label("Encrypted Text: ");
    encrypted_text.set_label_color(Color::White);
    encrypted_text.set_label_size(14);

    let mut decrypt_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 100)
        .with_label("Decrypt text");
    decrypt_button.set_color(Color::from_hex(0x545454));
    decrypt_button.set_label_color(Color::White);
    decrypt_button.set_label_size(16);

    let mut text_field = input::MultilineInput::default()
        .with_size(500, 240)
        .with_pos(150, 146);
    text_field.set_color(Color::from_hex(0x545454));
    text_field.set_text_color(Color::White);
    text_field.set_text_size(16);
    text_field.set_readonly(true);

    let mut error_label = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 400);
    error_label.set_label_color(Color::from_hex(0xFF3D3D));
    error_label.set_label_size(14);
    error_label.hide();

    main_window.end();
    main_window.redraw();

    back_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::menu::main_menu(main_window.clone(), Arc::clone(&program_data))
    });

    decrypt_button.set_callback({
        move |_| {
            let program_data_unlocked = program_data.lock().unwrap();

            for contact in &program_data_unlocked.contacts {
                if let Ok(message) =
                    from_encrypted::<String>(&encrypted_text_field.value(), &contact.contact_key)
                {
                    text_field.set_value(&format!(
                        "\nFrom: {}\n------------------------------------------\n{}\n------------------------------------------",
                        contact.contact_name, message
                    ));
                    return;
                }
            }

            error_label.set_label("Failed to decrypt!");
                error_label.show();
        }
    });
}
