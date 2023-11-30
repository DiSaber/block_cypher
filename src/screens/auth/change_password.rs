use crate::{
    program_data::ProgramData,
    screens::{self, builders},
};
use std::sync::{Arc, Mutex};

use fltk::{prelude::*, *};

pub fn change_password(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
    let mut built_change_password_menu = builders::build_change_password_menu(&mut main_window);

    built_change_password_menu.back_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::main_menu(main_window.clone(), Arc::clone(&program_data))
    });

    built_change_password_menu.confirm_button.set_callback({
        let password_field = built_change_password_menu.password_field.clone();
        let confirm_password_field = built_change_password_menu.confirm_password_field.clone();
        let mut error_label = built_change_password_menu.error_label.clone();
        let program_data = Arc::clone(&program_data);

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

            {
                let mut program_data_unlocked = program_data.lock().unwrap();

                program_data_unlocked.set_password(password_field.value().trim());

                program_data_unlocked.save_config();
            }

            screens::main_menu(main_window.clone(), Arc::clone(&program_data));
        }
    });
}
