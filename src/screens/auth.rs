use crate::utils::VERSION_CODE;
use std::io::{self, Write};

use console::Term;

pub fn setup(term: &Term) -> String {
    loop {
        print!(
            "Welcome to BlockCypher {VERSION_CODE}
            
            \rCreate your password: "
        );
        io::stdout().flush().unwrap();
        let password = term.read_secure_line().unwrap();
        print!("Confirm password: ");
        io::stdout().flush().unwrap();

        if password == term.read_secure_line().unwrap() {
            break password;
        }

        term.clear_screen().unwrap();
        println!("Passwords don't match!")
    }
}

pub fn returning(term: &Term) -> String {
    print!(
        "Welcome back to BlockCypher {VERSION_CODE}
    
        \rEnter your password: "
    );
    io::stdout().flush().unwrap();
    term.read_secure_line().unwrap()
}
