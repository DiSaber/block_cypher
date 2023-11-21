use std::sync::{Arc, Mutex};

use fltk::{enums::Color, prelude::*, *};

use crate::{program_data::ProgramData, utils::save_config};

use super::contacts;

pub fn edit_contact(
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
