use std::sync::{Arc, Mutex};

use fltk::{prelude::*, *};
use sha3::{Digest, Sha3_256};

use crate::{
    program_data::ProgramData,
    screens::{self, builders},
};

use super::{edit_contact, key_exchange};

pub fn contacts_menu(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
    let mut built_contacts_menu = builders::build_contacts_menu(&mut main_window, &program_data);

    built_contacts_menu.back_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::main_menu(main_window.clone(), Arc::clone(&program_data))
    });

    built_contacts_menu.contacts_dropdown.set_callback({
        let program_data = Arc::clone(&program_data);
        let contacts_dropdown = built_contacts_menu.contacts_dropdown.clone();
        let mut view_key = built_contacts_menu.view_key.clone();

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

    built_contacts_menu.edit_contact_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);
        let contacts_dropdown = built_contacts_menu.contacts_dropdown.clone();

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

    built_contacts_menu.add_contact_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| key_exchange(main_window.clone(), Arc::clone(&program_data))
    });
}
