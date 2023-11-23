use std::sync::{Arc, Mutex};

use fltk::{prelude::*, *};

use crate::{
    encryption_handler::from_encrypted,
    program_data::ProgramData,
    screens::{self, builders},
};

pub fn decrypt_file(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
    let mut built_decrypt_menu = builders::build_decrypt_menu(&mut main_window);

    built_decrypt_menu.back_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::main_menu(main_window.clone(), Arc::clone(&program_data))
    });

    built_decrypt_menu.decrypt_button.set_callback({
        move |_| {
            let program_data_unlocked = program_data.lock().unwrap();

            for contact in &program_data_unlocked.contacts {
                if let Ok(message) =
                    from_encrypted::<String>(&built_decrypt_menu.encrypted_text_field.value(), &contact.contact_key)
                {
                    built_decrypt_menu.text_field.set_value(&format!(
                        "\nFrom: {}\n------------------------------------------\n{}\n------------------------------------------",
                        contact.contact_name, message
                    ));
                    return;
                }
            }

            built_decrypt_menu.error_label.set_label("Failed to decrypt!");
            built_decrypt_menu.error_label.show();
        }
    });
}
