use std::sync::{Arc, Mutex};

use arboard::Clipboard;
use fltk::{prelude::*, *};

use crate::{
    encryption_handler::to_encrypted,
    program_data::ProgramData,
    screens::{self, builders},
};

pub fn encrypt_file(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
    let mut built_encrypt_menu = builders::build_encrypt_file_menu(&mut main_window, &program_data);

    built_encrypt_menu.back_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::main_menu(main_window.clone(), Arc::clone(&program_data))
    });

    built_encrypt_menu.encrypt_button.set_callback({
        let program_data = Arc::clone(&program_data);

        move |_| {
            if let Some(contact_name) = built_encrypt_menu.contacts_dropdown.choice() {
                let program_data_unlocked = program_data.lock().unwrap();
                let contact_index = program_data_unlocked
                    .contacts
                    .iter()
                    .position(|contact| contact.contact_name == contact_name)
                    .unwrap();

                let message = to_encrypted(
                    &built_encrypt_menu.text_field.value(),
                    &program_data_unlocked.contacts[contact_index].contact_key,
                )
                .unwrap();

                let mut clipboard = Clipboard::new().unwrap();
                let _ = clipboard.set_text(&message);
            }
        }
    });
}
