use std::sync::{Arc, Mutex};

use fltk::{enums::Color, prelude::*, *};

use crate::program_data::ProgramData;

pub struct BuiltEncryptFileMenu {
    pub back_button: button::Button,
    pub encrypt_button: button::Button,
    pub contacts_dropdown: menu::Choice,
    pub file_input: button::Button,
    pub selected_file_text: frame::Frame,
    pub error_label: frame::Frame,
}

pub fn build_encrypt_file_menu(
    main_window: &mut window::Window,
    program_data: &Arc<Mutex<ProgramData>>,
) -> BuiltEncryptFileMenu {
    main_window.clear();
    main_window.begin();

    let mut back_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(20, 20)
        .with_label("Back");
    back_button.set_color(Color::from_hex(0x545454));
    back_button.set_label_color(Color::White);
    back_button.set_label_size(16);

    let mut file_input = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 200)
        .with_label("Select File");
    file_input.set_color(Color::from_hex(0x545454));
    file_input.set_label_color(Color::White);
    file_input.set_label_size(16);

    let mut selected_file_text = frame::Frame::default()
        .with_size(700, 40)
        .with_pos(50, 230)
        .with_label("Selected File: ");
    selected_file_text.set_label_color(Color::White);
    selected_file_text.set_label_size(14);

    let program_data_unlocked = program_data.lock().unwrap();

    let mut contacts_dropdown = menu::Choice::default()
        .with_size(120, 30)
        .with_pos(340, 320);
    contacts_dropdown.add_choice(&program_data_unlocked.format_contacts());

    let mut contacts_count = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 350)
        .with_label(&format!(
            "{} {}",
            program_data_unlocked.contacts.len(),
            if program_data_unlocked.contacts.len() == 1 {
                "contact"
            } else {
                "contacts"
            }
        ));
    contacts_count.set_label_color(Color::White);
    contacts_count.set_label_size(14);

    let mut encrypt_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 400)
        .with_label("Encrypt and Save");
    encrypt_button.set_color(Color::from_hex(0x545454));
    encrypt_button.set_label_color(Color::White);
    encrypt_button.set_label_size(16);

    let mut error_label = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 430);
    error_label.set_label_color(Color::from_hex(0xFF3D3D));
    error_label.set_label_size(14);
    error_label.hide();

    main_window.end();
    main_window.redraw();

    BuiltEncryptFileMenu {
        back_button,
        encrypt_button,
        contacts_dropdown,
        file_input,
        selected_file_text,
        error_label,
    }
}
