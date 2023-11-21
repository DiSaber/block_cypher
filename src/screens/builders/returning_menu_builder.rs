use crate::utils::VERSION_CODE;

use fltk::{enums::Color, prelude::*, *};

pub struct BuiltReturningMenu {
    pub confirm_button: button::Button,
    pub password_field: input::SecretInput,
    pub error_label: frame::Frame,
}

pub fn build_returning_menu(main_window: &mut window::Window) -> BuiltReturningMenu {
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

    BuiltReturningMenu {
        confirm_button,
        password_field,
        error_label,
    }
}
