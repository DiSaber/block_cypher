use fltk::{enums::Color, prelude::*, *};

pub struct BuiltDecryptFileMenu {
    pub back_button: button::Button,
    pub decrypt_button: button::Button,
    pub file_input: button::Button,
    pub selected_file_text: frame::Frame,
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

    let mut decrypt_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 400)
        .with_label("Decrypt and Save");
    decrypt_button.set_color(Color::from_hex(0x545454));
    decrypt_button.set_label_color(Color::White);
    decrypt_button.set_label_size(16);

    let mut error_label = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 430);
    error_label.set_label_color(Color::from_hex(0xFF3D3D));
    error_label.set_label_size(14);
    error_label.hide();

    main_window.end();
    main_window.redraw();

    BuiltDecryptFileMenu {
        back_button,
        decrypt_button,
        file_input,
        selected_file_text,
        error_label,
    }
}
