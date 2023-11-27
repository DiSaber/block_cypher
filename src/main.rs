#![windows_subsystem = "windows"]

mod contact;
mod data_container;
mod encryption_handler;
mod file_container;
mod message_container;
mod program_data;
mod screens;
mod utils;

use data_container::DataContainer;
use directories::ProjectDirs;

use fltk::{enums::Color, prelude::*, *};
use fltk_theme::{SchemeType, WidgetScheme};
use std::{fs, path::Path};

fn main() {
    let data_path = ProjectDirs::from("com", "DiSaber", "BlockCypher").unwrap();
    let data_path: &Path = data_path.config_dir();
    let data_file = data_path.join("block_cypher.data");
    let data_file_contents = fs::read(data_file).unwrap_or_default();

    let app = app::App::default();
    let widget_scheme = WidgetScheme::new(SchemeType::Aqua);
    widget_scheme.apply();

    let mut main_window = window::Window::default()
        .with_size(800, 475)
        .with_label("Block Cypher");
    main_window.set_color(Color::from_hex(0x252525));

    if let Ok(data_container) = DataContainer::from_binary(&data_file_contents) {
        screens::returning(main_window.clone(), data_container)
    } else {
        screens::setup(main_window.clone())
    };

    app.run().unwrap();
}
