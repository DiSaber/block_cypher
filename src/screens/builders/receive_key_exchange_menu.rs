use fltk::{enums::Color, prelude::*, *};

pub struct BuiltReceiveKeyExchangeMenu {
    pub back_button: button::Button,
    pub copy_cipher_text_button: button::Button,
    pub receiving_key_field: input::Input,
    pub error_label: frame::Frame,
    pub add_contact_button: button::Button,
    pub contact_name_field: input::Input,
}

pub fn build_receive_key_exchange_menu(
    main_window: &mut window::Window,
) -> BuiltReceiveKeyExchangeMenu {
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

    let mut receiving_key_text = frame::Frame::default()
        .with_size(300, 24)
        .with_pos(25, 190)
        .with_label("Receiving Key: ");
    receiving_key_text.set_label_color(Color::White);
    receiving_key_text.set_label_size(14);

    let mut receiving_key_field = input::Input::default()
        .with_size(350, 24)
        .with_pos(225, 190);
    receiving_key_field.set_color(Color::from_hex(0x545454));
    receiving_key_field.set_text_color(Color::White);
    receiving_key_field.set_text_size(16);

    let mut copy_cipher_text_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 300)
        .with_label("Copy Cipher Text");
    copy_cipher_text_button.set_color(Color::from_hex(0x545454));
    copy_cipher_text_button.set_label_color(Color::White);
    copy_cipher_text_button.set_label_size(16);

    let mut description_text = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 230)
        .with_label("Enter the receiving key above and send the cipher text back");
    description_text.set_label_color(Color::White);
    description_text.set_label_size(14);

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

    BuiltReceiveKeyExchangeMenu {
        back_button,
        copy_cipher_text_button,
        receiving_key_field,
        error_label,
        add_contact_button,
        contact_name_field,
    }
}
