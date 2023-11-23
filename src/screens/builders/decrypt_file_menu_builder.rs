use fltk::{enums::Color, prelude::*, *};

pub struct BuiltDecryptFileMenu {
    pub back_button: button::Button,
    pub decrypt_button: button::Button,
    pub encrypted_text_field: input::Input,
    pub text_field: input::MultilineInput,
    pub error_label: frame::Frame,
}

pub fn build_decrypt_file_menu(main_window: &mut window::Window) -> BuiltDecryptFileMenu {
    main_window.clear();
    main_window.begin();

    let mut back_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(20, 20)
        .with_label("Back");
    back_button.set_color(Color::from_hex(0x545454));
    back_button.set_label_color(Color::White);
    back_button.set_label_size(16);

    let mut encrypted_text_field = input::Input::default().with_size(300, 24).with_pos(250, 60);
    encrypted_text_field.set_color(Color::from_hex(0x545454));
    encrypted_text_field.set_text_color(Color::White);
    encrypted_text_field.set_text_size(16);

    let mut encrypted_text = frame::Frame::default()
        .with_size(300, 24)
        .with_pos(50, 60)
        .with_label("Encrypted Text: ");
    encrypted_text.set_label_color(Color::White);
    encrypted_text.set_label_size(14);

    let mut decrypt_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 100)
        .with_label("Decrypt Text");
    decrypt_button.set_color(Color::from_hex(0x545454));
    decrypt_button.set_label_color(Color::White);
    decrypt_button.set_label_size(16);

    let mut text_field = input::MultilineInput::default()
        .with_size(500, 240)
        .with_pos(150, 146);
    text_field.set_color(Color::from_hex(0x545454));
    text_field.set_text_color(Color::White);
    text_field.set_text_size(16);
    text_field.set_readonly(true);

    let mut error_label = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 400);
    error_label.set_label_color(Color::from_hex(0xFF3D3D));
    error_label.set_label_size(14);
    error_label.hide();

    main_window.end();
    main_window.redraw();

    BuiltDecryptFileMenu {
        back_button,
        decrypt_button,
        encrypted_text_field,
        text_field,
        error_label,
    }
}
