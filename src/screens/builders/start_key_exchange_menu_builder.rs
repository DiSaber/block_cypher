use fltk::{enums::Color, prelude::*, *};

pub struct BuiltStartKeyExchangeMenu {
    pub back_button: button::Button,
    pub copy_public_key_button: button::Button,
    pub add_contact_button: button::Button,
    pub contact_name_field: input::Input,
    pub error_label: frame::Frame,
    pub cipher_text_field: input::Input,
}

pub fn build_start_key_exchange_menu(
    main_window: &mut window::Window,
) -> BuiltStartKeyExchangeMenu {
    main_window.clear();
    main_window.begin();

    let mut back_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(20, 20)
        .with_label("Back");
    back_button.set_color(Color::from_hex(0x545454));
    back_button.set_label_color(Color::White);
    back_button.set_label_size(16);

    let mut contact_name_text = frame::Frame::default()
        .with_size(300, 24)
        .with_pos(100, 150)
        .with_label("Contact Name: ");
    contact_name_text.set_label_color(Color::White);
    contact_name_text.set_label_size(14);

    let mut contact_name_field = input::Input::default()
        .with_size(200, 24)
        .with_pos(300, 150);
    contact_name_field.set_color(Color::from_hex(0x545454));
    contact_name_field.set_text_color(Color::White);
    contact_name_field.set_text_size(16);

    let mut copy_public_key_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 190)
        .with_label("Copy Receiving Key");
    copy_public_key_button.set_color(Color::from_hex(0x545454));
    copy_public_key_button.set_label_color(Color::White);
    copy_public_key_button.set_label_size(16);

    let mut description_text = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 230)
        .with_label("Send this receiving key to the other user and enter the cipher text they send back to you");
    description_text.set_label_color(Color::White);
    description_text.set_label_size(14);

    let mut cipher_text = frame::Frame::default()
        .with_size(300, 24)
        .with_pos(30, 300)
        .with_label("Cipher Text: ");
    cipher_text.set_label_color(Color::White);
    cipher_text.set_label_size(14);

    let mut cipher_text_field = input::Input::default()
        .with_size(350, 24)
        .with_pos(225, 300);
    cipher_text_field.set_color(Color::from_hex(0x545454));
    cipher_text_field.set_text_color(Color::White);
    cipher_text_field.set_text_size(16);

    let mut add_contact_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 350)
        .with_label("Add Contact");
    add_contact_button.set_color(Color::from_hex(0x545454));
    add_contact_button.set_label_color(Color::White);
    add_contact_button.set_label_size(16);

    let mut error_label = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 385);
    error_label.set_label_color(Color::from_hex(0xFF3D3D));
    error_label.set_label_size(14);
    error_label.hide();

    main_window.end();
    main_window.redraw();

    BuiltStartKeyExchangeMenu {
        back_button,
        copy_public_key_button,
        add_contact_button,
        contact_name_field,
        error_label,
        cipher_text_field,
    }
}
