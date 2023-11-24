use std::sync::{Arc, Mutex};

use fltk::{prelude::*, *};

use crate::{
    data_container::MessageContainer,
    encryption_handler::from_encrypted,
    program_data::ProgramData,
    screens::{self, builders},
};

pub fn decrypt(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
    let mut built_decrypt_menu = builders::build_decrypt_menu(&mut main_window);

    built_decrypt_menu.back_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::main_menu(main_window.clone(), Arc::clone(&program_data))
    });

    built_decrypt_menu.decrypt_button.set_callback({
        move |_| {
            let program_data_unlocked = program_data.lock().unwrap();

            let message_container = match MessageContainer::from_base64(&built_decrypt_menu.encrypted_text_field.value()) {
                Ok(message_container) => message_container,
                Err(_) => {
                    built_decrypt_menu.error_label.set_label("Invalid cipher text!");
                    built_decrypt_menu.error_label.show();
                    return;
                }
            };

            for contact in &program_data_unlocked.contacts {
                if !message_container.validate_user_id(&contact.contact_key) {
                    continue;
                }

                if let Ok(message) =
                    from_encrypted::<String>(&message_container.data_container, &contact.contact_key)
                {
                    built_decrypt_menu.text_field.set_value(&format!(
                        "\nFrom: {}\n------------------------------------------\n{}\n------------------------------------------",
                        contact.contact_name, message
                    ));
                    built_decrypt_menu.error_label.hide();
                    return;
                }
            }

            built_decrypt_menu.error_label.set_label("Failed to decrypt!");
            built_decrypt_menu.error_label.show();
        }
    });
}
