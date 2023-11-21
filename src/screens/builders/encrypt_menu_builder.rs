use std::sync::{Arc, Mutex};

use fltk::{enums::Color, prelude::*, *};

use crate::program_data::ProgramData;

pub struct BuiltEncryptMenu {
    pub back_button: button::Button,
    pub encrypt_button: button::Button,
    pub contacts_dropdown: menu::Choice,
    pub text_field: input::MultilineInput,
}

pub fn build_encrypt_menu(
    main_window: &mut window::Window,
    program_data: &Arc<Mutex<ProgramData>>,
) -> BuiltEncryptMenu {
    main_window.clear();
    main_window.begin();

    let mut back_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(20, 20)
        .with_label("Back");
    back_button.set_color(Color::from_hex(0x545454));
    back_button.set_label_color(Color::White);
    back_button.set_label_size(16);

    let mut text_field = input::MultilineInput::default()
        .with_size(500, 240)
        .with_pos(150, 60);
    text_field.set_color(Color::from_hex(0x545454));
    text_field.set_text_color(Color::White);
    text_field.set_text_size(16);

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
        .with_label("Copy encrypted text");
    encrypt_button.set_color(Color::from_hex(0x545454));
    encrypt_button.set_label_color(Color::White);
    encrypt_button.set_label_size(16);

    main_window.end();
    main_window.redraw();

    BuiltEncryptMenu {
        back_button,
        encrypt_button,
        contacts_dropdown,
        text_field,
    }
}
