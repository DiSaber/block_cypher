use std::sync::{Arc, Mutex};

use crate::screens;
use crate::utils::save_config;
use crate::{contact::Contact, program_data::ProgramData};

use fltk::{prelude::*, *};

use crate::screens::builders;

pub fn main_menu(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
    let mut built_main_menu = builders::build_main_menu(&mut main_window);

    {
        let mut program_data = program_data.lock().unwrap();

        if !program_data
            .contacts
            .iter()
            .any(|contact| contact.contact_name == "Personal (Built-in)")
        {
            program_data.contacts.push(Contact {
                contact_name: String::from("Personal (Built-in)"),
                contact_key: rand::random(),
            });

            save_config(&program_data, &program_data.hashed_password)
        }
    }

    built_main_menu.contacts_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::contacts_menu(main_window.clone(), Arc::clone(&program_data))
    });

    built_main_menu.encrypt_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::encrypt(main_window.clone(), Arc::clone(&program_data))
    });

    built_main_menu.encrypt_file_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::encrypt_file(main_window.clone(), Arc::clone(&program_data))
    });

    built_main_menu.decrypt_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::decrypt(main_window.clone(), Arc::clone(&program_data))
    });

    built_main_menu.decrypt_file_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::decrypt_file(main_window.clone(), Arc::clone(&program_data))
    });

    built_main_menu.change_password_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::change_password(main_window.clone(), Arc::clone(&program_data))
    });
}
