use crate::{
    encryption_handler::{from_encrypted, hash_password},
    program_data::ProgramData,
    screens,
    utils::{save_config, VERSION_CODE},
};
use std::sync::{Arc, Mutex};

use fltk::{enums::Color, prelude::*, *};

pub fn setup(mut main_window: window::Window) {
    let mut header = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 65)
        .with_label(&format!("Welcome to BlockCypher {VERSION_CODE}"));
    header.set_label_color(Color::White);
    header.set_label_size(24);

    let mut sub_header = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 105)
        .with_label("Create your password below");
    sub_header.set_label_color(Color::White);
    sub_header.set_label_size(14);

    let mut password_field = input::SecretInput::default()
        .with_size(300, 24)
        .with_pos(250, 206);
    password_field.set_color(Color::from_hex(0x545454));
    password_field.set_text_color(Color::White);
    password_field.set_text_size(16);

    let mut confirm_password_field = input::SecretInput::default()
        .with_size(300, 24)
        .with_pos(250, 246);
    confirm_password_field.set_color(Color::from_hex(0x545454));
    confirm_password_field.set_text_color(Color::White);
    confirm_password_field.set_text_size(16);

    let mut error_label = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 276);
    error_label.set_label_color(Color::from_hex(0xFF3D3D));
    error_label.set_label_size(14);
    error_label.hide();

    let mut confirm_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 361)
        .with_label("Confirm");
    confirm_button.set_color(Color::from_hex(0x545454));
    confirm_button.set_label_color(Color::White);
    confirm_button.set_label_size(16);

    main_window.end();
    main_window.show();

    confirm_button.set_callback({
        let password_field = password_field.clone();
        let confirm_password_field = confirm_password_field.clone();
        let mut error_label = error_label.clone();

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

            screens::menu::main_menu(main_window.clone(), Arc::new(Mutex::new(program_data)));
        }
    });
}

pub fn returning(mut main_window: window::Window, data_file_contents: String) {
    let mut header = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 65)
        .with_label(&format!("Welcome Back to BlockCypher {VERSION_CODE}"));
    header.set_label_color(Color::White);
    header.set_label_size(24);

    let mut sub_header = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 105)
        .with_label("Enter your password below");
    sub_header.set_label_color(Color::White);
    sub_header.set_label_size(14);

    let mut password_field = input::SecretInput::default()
        .with_size(300, 24)
        .with_pos(250, 206);
    password_field.set_color(Color::from_hex(0x545454));
    password_field.set_text_color(Color::White);
    password_field.set_text_size(16);

    let mut error_label = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 236);
    error_label.set_label_color(Color::from_hex(0xFF3D3D));
    error_label.set_label_size(14);
    error_label.hide();

    let mut confirm_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 321)
        .with_label("Confirm");
    confirm_button.set_color(Color::from_hex(0x545454));
    confirm_button.set_label_color(Color::White);
    confirm_button.set_label_size(16);

    main_window.end();
    main_window.show();

    confirm_button.set_callback({
        let password_field = password_field.clone();
        let mut error_label = error_label.clone();

        move |_| {
            if password_field.value().trim().is_empty() {
                error_label.set_label("Password cannot be empty!");
                error_label.show();
                return;
            }

            let password = hash_password(password_field.value().trim());

            match from_encrypted::<ProgramData>(&data_file_contents, &password) {
                Ok(program_data) => screens::menu::main_menu(
                    main_window.clone(),
                    Arc::new(Mutex::new(program_data)),
                ),
                Err(_) => {
                    error_label.set_label("Invalid password!");
                    error_label.show();
                }
            }
        }
    });
}

pub fn change_password(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
    main_window.clear();
    main_window.begin();

    let mut back_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(20, 20)
        .with_label("Back");
    back_button.set_color(Color::from_hex(0x545454));
    back_button.set_label_color(Color::White);
    back_button.set_label_size(16);

    let mut sub_header = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 105)
        .with_label("Create your new password");
    sub_header.set_label_color(Color::White);
    sub_header.set_label_size(14);

    let mut password_field = input::SecretInput::default()
        .with_size(300, 24)
        .with_pos(250, 206);
    password_field.set_color(Color::from_hex(0x545454));
    password_field.set_text_color(Color::White);
    password_field.set_text_size(16);

    let mut confirm_password_field = input::SecretInput::default()
        .with_size(300, 24)
        .with_pos(250, 246);
    confirm_password_field.set_color(Color::from_hex(0x545454));
    confirm_password_field.set_text_color(Color::White);
    confirm_password_field.set_text_size(16);

    let mut error_label = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 276);
    error_label.set_label_color(Color::from_hex(0xFF3D3D));
    error_label.set_label_size(14);
    error_label.hide();

    let mut confirm_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 361)
        .with_label("Confirm");
    confirm_button.set_color(Color::from_hex(0x545454));
    confirm_button.set_label_color(Color::White);
    confirm_button.set_label_size(16);

    main_window.end();
    main_window.show();

    back_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::menu::main_menu(main_window.clone(), Arc::clone(&program_data))
    });

    confirm_button.set_callback({
        let password_field = password_field.clone();
        let confirm_password_field = confirm_password_field.clone();
        let mut error_label = error_label.clone();
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

                program_data_unlocked.hashed_password =
                    hash_password(password_field.value().trim());

                save_config(
                    &program_data_unlocked,
                    &program_data_unlocked.hashed_password,
                );
            }

            screens::menu::main_menu(main_window.clone(), Arc::clone(&program_data));
        }
    });
}
