use crate::{
    encryption_handler::hash_password,
    program_data::ProgramData,
    screens::{self, builders},
    utils::save_config,
};
use std::sync::{Arc, Mutex};

use fltk::{prelude::*, *};

pub fn setup(mut main_window: window::Window) {
    let mut built_setup_menu = builders::build_setup_menu(&mut main_window);

    built_setup_menu.confirm_button.set_callback({
        let password_field = built_setup_menu.password_field.clone();
        let confirm_password_field = built_setup_menu.confirm_password_field.clone();
        let mut error_label = built_setup_menu.error_label.clone();

        move |_| {
            if password_field.value() != confirm_password_field.value() {
                error_label.set_label("Passwords don't match!");
                error_label.show();
                return;
            }

            if password_field.value().trim().is_empty() {
                error_label.set_label("Password cannot be empty!");
                error_label.show();
                return;
            }

            let password = hash_password(password_field.value().trim());
            let program_data = ProgramData::new(&password);

            save_config(&program_data, &password);

            screens::main_menu(main_window.clone(), Arc::new(Mutex::new(program_data)));
        }
    });
}
