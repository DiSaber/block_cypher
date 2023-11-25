use std::{
    fs,
    path::PathBuf,
    sync::{Arc, Mutex},
};

use fltk::{prelude::*, *};

use crate::{
    data_container::MessageContainer,
    file_encryption_handler::from_encrypted,
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

            match selected_file.lock() {
                Ok(mut selected_file) => *selected_file = file_path,
                Err(_) => {
                    error_label.set_label("Failed to load file!");
                    error_label.show()
                }
            }
        }
    });

    built_decrypt_file_menu.decrypt_button.set_callback({
        move |_| {
            let selected_file = match selected_file.lock() {
                Ok(selected_file) => selected_file,
                Err(_) => {
                    built_decrypt_file_menu
                        .error_label
                        .set_label("Failed to load file!");
                    built_decrypt_file_menu.error_label.show();
                    return;
                }
            };

            let file = match fs::read(&*selected_file) {
                Ok(file) => file,
                Err(_) => {
                    built_decrypt_file_menu
                        .error_label
                        .set_label("Failed to load file!");
                    built_decrypt_file_menu.error_label.show();
                    return;
                }
            };

            let message_container = match MessageContainer::from_binary(&file) {
                Ok(message_container) => message_container,
                Err(_) => {
                    built_decrypt_file_menu
                        .error_label
                        .set_label("Invalid encrypted file!");
                    built_decrypt_file_menu.error_label.show();
                    return;
                }
            };

            let program_data_unlocked = program_data.lock().unwrap();

            for contact in &program_data_unlocked.contacts {
                if !message_container.validate_user_id(&contact.contact_key) {
                    continue;
                }

                if let Ok(file_container) =
                    from_encrypted(&message_container.data_container, &contact.contact_key)
                {
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

                    match fs::write(file_path, file_container.file) {
                        Ok(_) => (),
                        Err(_) => {
                            built_decrypt_file_menu
                                .error_label
                                .set_label("Failed to save file!");
                            built_decrypt_file_menu.error_label.show();
                            return;
                        }
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
