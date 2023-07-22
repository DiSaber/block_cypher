mod contact;
mod data_container;
mod encryption_handler;
mod program_data;
mod screens;
mod utils;

use directories::ProjectDirs;
use encryption_handler::{from_encrypted, hash_password};
use program_data::ProgramData;

use console::{Key, Term};
use std::{fs, path::Path};

use utils::{save_config, VERSION_CODE};

fn main() {
    let term = Term::stdout();

    let data_path = ProjectDirs::from("com", "DiSaber", "BlockCypher").unwrap();
    let data_path: &Path = data_path.config_dir();
    let data_file = data_path.join("block_cypher.dat");

    let data_file_contents = fs::read_to_string(&data_file).unwrap_or_default();

    let mut program_data = if data_file_contents.trim().is_empty() {
        let password = hash_password(screens::auth::setup(&term).trim());
        let program_data = ProgramData::new(&password);

        save_config(&program_data, &password);
        program_data
    } else {
        loop {
            let password = hash_password(screens::auth::returning(&term).trim());

            if let Ok(program_data) = from_encrypted::<ProgramData>(&data_file_contents, &password)
            {
                break program_data;
            }

            term.clear_screen().unwrap();
            println!("Invalid password!")
        }
    };

    loop {
        term.clear_screen().unwrap();
        println!(
            "BlockCypher {VERSION_CODE}

            \rPress the 'e' key to encrypt a message
            \rPress the 'd' key to decrypt a recieved message
            \rPress the 'c' key to enter the contacts menu

            \rPress the escape key to terminate your session"
        );

        let key = term.read_key().unwrap_or(Key::Alt);

        match key {
            Key::Char('e') => screens::cipher::encrypt(&term, &program_data),
            Key::Char('d') => screens::cipher::decrypt(&term, &program_data),
            Key::Char('c') => screens::contacts::contacts(&term, &mut program_data),
            Key::Escape => {
                term.clear_screen().unwrap();
                return;
            }
            _ => (),
        };
    }
}
