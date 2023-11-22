use std::sync::{Arc, Mutex};

use fltk::{enums::Color, prelude::*, *};

use crate::program_data::ProgramData;

pub struct BuiltContactsMenu {
    pub back_button: button::Button,
    pub contacts_dropdown: menu::Choice,
    pub view_key: frame::Frame,
    pub edit_contact_button: button::Button,
    pub add_contact_button: button::Button,
}

pub fn build_contacts_menu(
    main_window: &mut window::Window,
    program_data: &Arc<Mutex<ProgramData>>,
) -> BuiltContactsMenu {
    main_window.clear();
    main_window.begin();

    let mut back_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(20, 20)
        .with_label("Back");
    back_button.set_color(Color::from_hex(0x545454));
    back_button.set_label_color(Color::White);
    back_button.set_label_size(16);

    let mut view_key = frame::Frame::default()
        .with_size(150, 60)
        .with_pos(150, 127);
    view_key.set_label_color(Color::White);
    view_key.set_label_size(14);

    let program_data_unlocked = program_data.lock().unwrap();

    let mut contacts_dropdown = menu::Choice::default()
        .with_size(120, 30)
        .with_pos(340, 142);
    contacts_dropdown.add_choice(&program_data_unlocked.format_contacts());

    let mut contacts_count = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 172)
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

    let mut edit_contact_button = button::Button::default()
        .with_size(100, 20)
        .with_pos(490, 147)
        .with_label("Edit Contact");
    edit_contact_button.set_color(Color::from_hex(0x545454));
    edit_contact_button.set_label_color(Color::White);
    edit_contact_button.set_label_size(12);

    let mut add_contact_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 290)
        .with_label("Add Contact");
    add_contact_button.set_color(Color::from_hex(0x545454));
    add_contact_button.set_label_color(Color::White);
    add_contact_button.set_label_size(16);

    main_window.end();
    main_window.redraw();

    BuiltContactsMenu {
        back_button,
        contacts_dropdown,
        view_key,
        edit_contact_button,
        add_contact_button,
    }
}
