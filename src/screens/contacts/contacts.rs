use std::sync::{Arc, Mutex};

use fltk::{enums::Color, prelude::*, *};
use sha3::{Digest, Sha3_256};

use crate::{program_data::ProgramData, screens};

use super::{edit_contact, key_exchange};

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

        move |_| key_exchange(main_window.clone(), Arc::clone(&program_data))
    });
}
