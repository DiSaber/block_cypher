use fltk::{enums::Color, prelude::*, *};

pub struct BuiltEditContactMenu {
    pub back_button: button::Button,
    pub save_contact_button: button::Button,
    pub contact_name_field: input::Input,
    pub error_label: frame::Frame,
    pub delete_contact_button: button::Button,
}

pub fn build_edit_contact_menu(
    main_window: &mut window::Window,
    contact_name: &str,
) -> BuiltEditContactMenu {
    main_window.clear();
    main_window.begin();

    let mut back_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(20, 20)
        .with_label("Back");
    back_button.set_color(Color::from_hex(0x545454));
    back_button.set_label_color(Color::White);
    back_button.set_label_size(16);

    let mut contact_name_field = input::Input::default()
        .with_size(200, 24)
        .with_pos(300, 150);
    contact_name_field.set_color(Color::from_hex(0x545454));
    contact_name_field.set_text_color(Color::White);
    contact_name_field.set_text_size(16);
    contact_name_field.set_value(contact_name);

    let mut contact_name_text = frame::Frame::default()
        .with_size(300, 24)
        .with_pos(100, 150)
        .with_label("Contact Name: ");
    contact_name_text.set_label_color(Color::White);
    contact_name_text.set_label_size(14);

    let mut save_contact_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 200)
        .with_label("Save Contact");
    save_contact_button.set_color(Color::from_hex(0x545454));
    save_contact_button.set_label_color(Color::White);
    save_contact_button.set_label_size(16);

    let mut error_label = frame::Frame::default()
        .with_size(300, 40)
        .with_pos(250, 235);
    error_label.set_label_color(Color::from_hex(0xFF3D3D));
    error_label.set_label_size(14);
    error_label.hide();

    let mut delete_contact_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(325, 300)
        .with_label("Delete Contact");
    delete_contact_button.set_color(Color::from_hex(0x545454));
    delete_contact_button.set_label_color(Color::White);
    delete_contact_button.set_label_size(16);

    main_window.end();
    main_window.redraw();

    BuiltEditContactMenu {
        back_button,
        save_contact_button,
        contact_name_field,
        error_label,
        delete_contact_button,
    }
}
