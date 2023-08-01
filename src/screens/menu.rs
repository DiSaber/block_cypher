use std::sync::{Arc, Mutex};

use crate::program_data::ProgramData;
use crate::screens;
use crate::VERSION_CODE;

use fltk::{enums::Color, prelude::*, *};

pub fn main_menu(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
    main_window.clear();
    main_window.begin();

    let mut header = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 65)
        .with_label(&format!("BlockCypher {VERSION_CODE}"));
    header.set_label_color(Color::White);
    header.set_label_size(24);

    let mut contacts_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 200)
        .with_label("Contacts");
    contacts_button.set_color(Color::from_hex(0x545454));
    contacts_button.set_label_color(Color::White);
    contacts_button.set_label_size(16);

    let mut encrypt_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 250)
        .with_label("Encrypt");
    encrypt_button.set_color(Color::from_hex(0x545454));
    encrypt_button.set_label_color(Color::White);
    encrypt_button.set_label_size(16);

    let mut decrypt_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 300)
        .with_label("Decrypt");
    decrypt_button.set_color(Color::from_hex(0x545454));
    decrypt_button.set_label_color(Color::White);
    decrypt_button.set_label_size(16);

    let mut change_password_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 400)
        .with_label("Change Password");
    change_password_button.set_color(Color::from_hex(0x545454));
    change_password_button.set_label_color(Color::White);
    change_password_button.set_label_size(16);

    contacts_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::contacts::contacts(main_window.clone(), Arc::clone(&program_data))
    });

    encrypt_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::cipher::encrypt(main_window.clone(), Arc::clone(&program_data))
    });

    decrypt_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::cipher::decrypt(main_window.clone(), Arc::clone(&program_data))
    });

    change_password_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| screens::auth::change_password(main_window.clone(), Arc::clone(&program_data))
    });

    main_window.end();
    main_window.redraw();
}
