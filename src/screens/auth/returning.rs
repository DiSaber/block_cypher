use crate::{
    data_container::DataContainer,
    encryption_handler::{from_encrypted, hash_password},
    program_data::ProgramData,
    screens::{self, builders},
};
use std::sync::{Arc, Mutex};

use fltk::{prelude::*, *};

pub fn returning(mut main_window: window::Window, data_container: DataContainer) {
    let mut built_returning_menu = builders::build_returning_menu(&mut main_window);

    built_returning_menu.confirm_button.set_callback({
        let password_field = built_returning_menu.password_field.clone();
        let mut error_label = built_returning_menu.error_label.clone();

        move |_| {
            if password_field.value().trim().is_empty() {
                error_label.set_label("Password cannot be empty!");
                error_label.show();
                return;
            }

            let password = hash_password(password_field.value().trim());

            match from_encrypted::<ProgramData>(&data_container, &password) {
                Ok(program_data) => {
                    screens::main_menu(main_window.clone(), Arc::new(Mutex::new(program_data)))
                }
                Err(_) => {
                    error_label.set_label("Invalid password!");
                    error_label.show();
                }
            }
        }
    });
}
