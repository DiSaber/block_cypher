use fltk::{enums::Color, prelude::*, *};

pub struct BuiltKeyExchangeMenu {
    pub back_button: button::Button,
    pub start_key_exchange_button: button::Button,
    pub receive_key_button: button::Button,
}

pub fn build_key_exchange_menu(main_window: &mut window::Window) -> BuiltKeyExchangeMenu {
    main_window.clear();
    main_window.begin();

    let mut back_button = button::Button::default()
        .with_size(150, 30)
        .with_pos(20, 20)
        .with_label("Back");
    back_button.set_color(Color::from_hex(0x545454));
    back_button.set_label_color(Color::White);
    back_button.set_label_size(16);

    let mut start_key_exchange_button = button::Button::default()
        .with_size(175, 30)
        .with_pos(312, 207)
        .with_label("Start Key Exchange");
    start_key_exchange_button.set_color(Color::from_hex(0x545454));
    start_key_exchange_button.set_label_color(Color::White);
    start_key_exchange_button.set_label_size(16);

    let mut receive_key_button = button::Button::default()
        .with_size(175, 30)
        .with_pos(312, 257)
        .with_label("Enter a Receiving Key");
    receive_key_button.set_color(Color::from_hex(0x545454));
    receive_key_button.set_label_color(Color::White);
    receive_key_button.set_label_size(16);

    main_window.end();
    main_window.redraw();

    BuiltKeyExchangeMenu {
        back_button,
        start_key_exchange_button,
        receive_key_button,
    }
}
