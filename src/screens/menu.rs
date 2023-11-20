use std::sync::{Arc, Mutex};

use crate::program_data::ProgramData;
use crate::screens;

use fltk::{prelude::*, *};

use crate::screens::builders::main_menu;

pub fn main_menu(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
    let mut built_main_menu = main_menu::build_main_menu(&mut main_window);

    built_main_menu.contacts_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::contacts::contacts(main_window.clone(), Arc::clone(&program_data))
    });

    built_main_menu.encrypt_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::cipher::encrypt(main_window.clone(), Arc::clone(&program_data))
    });

    built_main_menu.decrypt_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::cipher::decrypt(main_window.clone(), Arc::clone(&program_data))
    });

    built_main_menu.change_password_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::auth::change_password(main_window.clone(), Arc::clone(&program_data))
    });
}
