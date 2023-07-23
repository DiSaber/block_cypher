use crate::{
    encryption_handler::{from_encrypted, hash_password},
    program_data::ProgramData,
    utils::{save_config, VERSION_CODE},
};
use std::io::{self, Write};

use console::{style, Term};

pub fn setup(term: &Term) -> ProgramData {
    loop {
        print!(
            "Welcome to BlockCypher {VERSION_CODE}
            
            \rCreate your password: "
        );
        io::stdout().flush().unwrap();
        let password = term.read_secure_line().unwrap();
        let password = password.trim();

        if password.is_empty() {
            term.clear_screen().unwrap();
            println!("Password cannot be empty!");
            continue;
        }

        print!("Confirm your password: ");
        io::stdout().flush().unwrap();

        if password == term.read_secure_line().unwrap().trim() {
            let password = hash_password(password);
            let program_data = ProgramData::new(&password);

            save_config(&program_data, &password);
            return program_data;
        }

        term.clear_screen().unwrap();
        println!("{}", style("Passwords don't match!").red())
    }
}

pub fn returning(term: &Term, data_file_contents: &String) -> ProgramData {
    loop {
        print!(
            "Welcome back to BlockCypher {VERSION_CODE}
    
        \rEnter your password: "
        );
        io::stdout().flush().unwrap();
        let password = term.read_secure_line().unwrap();
        let password = password.trim();

        let password = hash_password(password);

        if let Ok(program_data) = from_encrypted::<ProgramData>(data_file_contents, &password) {
            return program_data;
        }

        term.clear_screen().unwrap();
        println!("{}", style("Invalid password!").red())
    }
}

pub fn change(term: &Term, program_data: &mut ProgramData) {
    term.clear_screen().unwrap();

    loop {
        print!("Create your new password (leave empty to exit): ");
        io::stdout().flush().unwrap();
        let password = term.read_secure_line().unwrap();
        let password = password.trim();

        if password.is_empty() {
            return;
        }

        print!("Confirm your new password: ");
        io::stdout().flush().unwrap();

        if password == term.read_secure_line().unwrap().trim() {
            let password = hash_password(password);
            program_data.hashed_password = password;

            save_config(&program_data, &password);
            return;
        }

        term.clear_screen().unwrap();
        println!("{}", style("Passwords don't match!").red())
    }
}
