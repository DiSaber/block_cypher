use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use fltk::{prelude::*, *};

use crate::{
    encryption_handler::from_encrypted,
    file_container::FileContainer,
    message_container::MessageContainer,
    program_data::ProgramData,
    screens::{self, builders},
};

pub fn decrypt_file(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
    let mut built_decrypt_file_menu = builders::build_decrypt_file_menu(&mut main_window);

    built_decrypt_file_menu.back_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::main_menu(main_window.clone(), Arc::clone(&program_data))
    });

    let selected_file: Arc<Mutex<PathBuf>> = Arc::new(Mutex::new(PathBuf::default()));

    built_decrypt_file_menu.file_input.set_callback({
        let mut selected_file_text = built_decrypt_file_menu.selected_file_text.clone();
        let mut error_label = built_decrypt_file_menu.error_label.clone();
        let selected_file = Arc::clone(&selected_file);

        move |_| {
            let mut nfc = dialog::NativeFileChooser::new(dialog::FileDialogType::BrowseFile);
            nfc.show();
            nfc.set_filter("");
            let file_path = nfc.filename();
            selected_file_text.set_label(&format!("Selected File: {}", file_path.display()));

            if let Ok(mut selected_file) = selected_file.lock() {
                *selected_file = file_path;
            } else {
                error_label.set_label("Failed to load file!");
                error_label.show();
            }
        }
    });

    built_decrypt_file_menu.decrypt_button.set_callback({
        move |_| {
            let Ok(selected_file) = selected_file.lock() else {
                built_decrypt_file_menu
                    .error_label
                    .set_label("Failed to load file!");
                built_decrypt_file_menu.error_label.show();
                return;
            };

            let Ok(file) = fs::read(&*selected_file) else {
                built_decrypt_file_menu
                    .error_label
                    .set_label("Failed to load file!");
                built_decrypt_file_menu.error_label.show();
                return;
            };

            let Ok(message_container) = MessageContainer::from_binary(&file) else {
                built_decrypt_file_menu
                    .error_label
                    .set_label("Invalid encrypted file!");
                built_decrypt_file_menu.error_label.show();
                return;
            };

            let program_data_unlocked = program_data.lock().unwrap();

            for contact in &program_data_unlocked.contacts {
                if !message_container.validate_user_id(&contact.contact_key) {
                    continue;
                }

                if let Ok(file_container) = from_encrypted::<FileContainer>(
                    &message_container.data_container,
                    &contact.contact_key,
                ) {
                    let mut nfc =
                        dialog::NativeFileChooser::new(dialog::FileDialogType::BrowseSaveFile);
                    nfc.set_title("Save Decrypted File");
                    nfc.set_preset_file(&format!(
                        "(From - {}) {}",
                        contact.contact_name, &file_container.filename
                    ));
                    nfc.set_filter("");
                    nfc.show();
                    let file_path = nfc.filename();

                    if let Err(_) = fs::write(file_path, file_container.file) {
                        built_decrypt_file_menu
                            .error_label
                            .set_label("Failed to save file!");
                        built_decrypt_file_menu.error_label.show();
                        return;
                    };

                    built_decrypt_file_menu.error_label.hide();
                    return;
                }
            }

            built_decrypt_file_menu
                .error_label
                .set_label("Failed to decrypt!");
            built_decrypt_file_menu.error_label.show();
        }
    });
}
