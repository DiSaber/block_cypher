mod contact;
mod data_container;
mod encryption_handler;
mod program_data;
mod screens;
mod utils;

use arboard::Clipboard;
use directories::ProjectDirs;

use console::{style, Key, Term};
use std::{fs, path::Path};

use utils::VERSION_CODE;

fn main() {
    let term = Term::stdout();
    let mut clipboard = Clipboard::new().unwrap();

    let data_path = ProjectDirs::from("com", "DiSaber", "BlockCypher").unwrap();
    let data_path: &Path = data_path.config_dir();
    let data_file = data_path.join("block_cypher.dat");

    let data_file_contents = fs::read_to_string(&data_file).unwrap_or_default();

    let mut program_data = if data_file_contents.trim().is_empty() {
        screens::auth::setup(&term)
    } else {
        screens::auth::returning(&term, &data_file_contents)
    };

    loop {
        term.clear_screen().unwrap();
        println!(
            "BlockCypher {VERSION_CODE}

            \rPress the {} key to encrypt a message
            \rPress the {} key to decrypt a recieved message
            \rPress the {} key to enter the contacts menu

            \rPress the {} key to change your password


            \rPress the {} key to terminate your session",
            style("'e'").cyan(),
            style("'d'").cyan(),
            style("'c'").cyan(),
            style("'p'").cyan(),
            style("escape").red()
        );

        let key = term.read_key().unwrap_or(Key::Unknown);

        match key {
            Key::Char('e') => screens::cipher::encrypt(&term, &mut clipboard, &program_data),
            Key::Char('d') => screens::cipher::decrypt(&term, &program_data),
            Key::Char('c') => screens::contacts::contacts(&term, &mut clipboard, &mut program_data),
            Key::Char('p') => screens::auth::change(&term, &mut program_data),
            Key::Escape => {
                term.clear_screen().unwrap();
                return;
            }
            _ => (),
        };
    }
}
