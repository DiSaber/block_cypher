use std::sync::{Arc, Mutex};

use fltk::{prelude::*, *};

use crate::{program_data::ProgramData, screens::builders};

use super::contacts_menu;

pub fn edit_contact(
    mut main_window: window::Window,
    program_data: Arc<Mutex<ProgramData>>,
    contact_name: &str,
) {
    let mut built_edit_contact_menu =
        builders::build_edit_contact_menu(&mut main_window, contact_name);

    let program_data_unlocked = program_data.lock().unwrap();
    let contact_index = program_data_unlocked
        .contacts
        .iter()
        .position(|contact| contact.contact_name == contact_name)
        .unwrap();

    built_edit_contact_menu.back_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| contacts_menu(main_window.clone(), Arc::clone(&program_data))
    });

    built_edit_contact_menu.save_contact_button.set_callback({
        let main_window = main_window.clone();
        let contact_name_field = built_edit_contact_menu.contact_name_field.clone();
        let mut error_label = built_edit_contact_menu.error_label.clone();
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

                program_data_unlocked.save_config();
            }

            contacts_menu(main_window.clone(), Arc::clone(&program_data));
        }
    });

    built_edit_contact_menu.delete_contact_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| {
            {
                let mut program_data_unlocked = program_data.lock().unwrap();

                program_data_unlocked.contacts.remove(contact_index);

                program_data_unlocked.save_config();
            }

            contacts_menu(main_window.clone(), Arc::clone(&program_data));
        }
    });
}
