use std::sync::{Arc, Mutex};

use fltk::{prelude::*, *};

use crate::{program_data::ProgramData, screens::builders};

use super::{contacts, receive_key_exchange, start_key_exchange};

pub fn key_exchange(mut main_window: window::Window, program_data: Arc<Mutex<ProgramData>>) {
    let mut built_key_exchange_menu = builders::build_key_exchange_menu(&mut main_window);

    built_key_exchange_menu.back_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| contacts(main_window.clone(), Arc::clone(&program_data))
    });

    built_key_exchange_menu
        .start_key_exchange_button
        .set_callback({
            let main_window = main_window.clone();
            let program_data = Arc::clone(&program_data);

            move |_| start_key_exchange(main_window.clone(), Arc::clone(&program_data))
        });

    built_key_exchange_menu.receive_key_button.set_callback({
        let main_window = main_window.clone();
        let program_data = Arc::clone(&program_data);

        move |_| receive_key_exchange(main_window.clone(), Arc::clone(&program_data))
    });
}
