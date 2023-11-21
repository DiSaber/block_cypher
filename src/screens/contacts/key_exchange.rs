use std::sync::{Arc, Mutex};

use fltk::{enums::Color, prelude::*, *};

use crate::program_data::ProgramData;

use super::{contacts, recieve_key_exchange, start_key_exchange};

pub fn key_exchange(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
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

    back_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| contacts(main_window.clone(), Arc::clone(&program_data))
    });

    start_key_exchange_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| start_key_exchange(main_window.clone(), Arc::clone(&program_data))
    });

    receive_key_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| recieve_key_exchange(main_window.clone(), Arc::clone(&program_data))
    });
}
