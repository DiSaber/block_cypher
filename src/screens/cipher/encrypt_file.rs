use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use fltk::{prelude::*, *};

use crate::{
    data_container::MessageContainer,
    file_container::FileContainer,
    file_encryption_handler::to_encrypted,
    program_data::ProgramData,
    screens::{self, builders},
};

pub fn encrypt_file(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
    let mut built_encrypt_file_menu =
        builders::build_encrypt_file_menu(&mut main_window, &program_data);

    built_encrypt_file_menu.back_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::main_menu(main_window.clone(), Arc::clone(&program_data))
    });

    let selected_file: Arc<Mutex<PathBuf>> = Arc::new(Mutex::new(PathBuf::default()));

    built_encrypt_file_menu.file_input.set_callback({
        let mut selected_file_text = built_encrypt_file_menu.selected_file_text.clone();
        let mut error_label = built_encrypt_file_menu.error_label.clone();
        let selected_file = Arc::clone(&selected_file);

        move |_| {
            let mut nfc = dialog::NativeFileChooser::new(dialog::FileDialogType::BrowseFile);
            nfc.show();
            nfc.set_filter("");
            let file_path = nfc.filename();
            selected_file_text.set_label(&format!("Selected File: {}", file_path.display()));

            match selected_file.lock() {
                Ok(mut selected_file) => *selected_file = file_path,
                Err(_) => {
                    error_label.set_label("Failed to load file!");
                    error_label.show()
                }
            }
        }
    });

    built_encrypt_file_menu.encrypt_button.set_callback({
        move |_| {
            let contact_name = match built_encrypt_file_menu.contacts_dropdown.choice() {
                Some(contact_name) => contact_name,
                None => {
                    built_encrypt_file_menu
                        .error_label
                        .set_label("No contact selected!");
                    built_encrypt_file_menu.error_label.show();
                    return;
                }
            };

            let program_data_unlocked = program_data.lock().unwrap();
            let contact_index = program_data_unlocked
                .contacts
                .iter()
                .position(|contact| contact.contact_name == contact_name)
                .unwrap();

            let selected_file = match selected_file.lock() {
                Ok(selected_file) => selected_file,
                Err(_) => {
                    built_encrypt_file_menu
                        .error_label
                        .set_label("Failed to load file!");
                    built_encrypt_file_menu.error_label.show();
                    return;
                }
            };

            let file = match fs::read(&*selected_file) {
                Ok(file) => file,
                Err(_) => {
                    built_encrypt_file_menu
                        .error_label
                        .set_label("Failed to load file!");
                    built_encrypt_file_menu.error_label.show();
                    return;
                }
            };

            let message_container = MessageContainer::new(
                to_encrypted(
                    &FileContainer {
                        file,
                        filename: selected_file
                            .file_name()
                            .unwrap_or_default()
                            .to_string_lossy()
                            .to_string(),
                    },
                    &program_data_unlocked.contacts[contact_index].contact_key,
                )
                .unwrap(),
                &program_data_unlocked.contacts[contact_index].contact_key,
            );

            let mut nfc = dialog::NativeFileChooser::new(dialog::FileDialogType::BrowseSaveFile);
            nfc.set_title("Save Encrypted File");
            nfc.set_preset_file("encrypted.data");
            nfc.set_filter("");
            nfc.show();
            let file_path = nfc.filename();

            match fs::write(file_path, message_container.to_binary()) {
                Ok(_) => (),
                Err(_) => {
                    built_encrypt_file_menu
                        .error_label
                        .set_label("Failed to save file!");
                    built_encrypt_file_menu.error_label.show();
                    return;
                }
            };

            built_encrypt_file_menu.error_label.hide();
        }
    });
}
