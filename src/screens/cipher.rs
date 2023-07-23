use std::io::{self, Write};

use arboard::Clipboard;
use console::{style, Term};

use crate::{
    encryption_handler::{from_encrypted, to_encrypted},
    program_data::ProgramData,
};

pub fn encrypt(term: &Term, clipboard: &mut Clipboard, program_data: &ProgramData) {
    term.clear_screen().unwrap();

    loop {
        print!(
            "{}
            \r{} {}

            \rContact name (leave empty to exit): ",
            program_data.format_contacts(),
            program_data.contacts.len(),
            if program_data.contacts.len() == 1 {
                "contact"
            } else {
                "contacts"
            }
        );
        io::stdout().flush().unwrap();

        let contact_name = term.read_line().unwrap_or_default();
        let contact_name = contact_name.trim().to_lowercase();

        if contact_name.is_empty() {
            return;
        }

        let contact = match program_data
            .contacts
            .iter()
            .find(|contact| contact.contact_name == contact_name)
        {
            Some(contact) => contact,
            None => {
                term.clear_screen().unwrap();
                println!("{}", style("Could not find the contact!").red());
                continue;
            }
        };

        print!("Message to encrypt: ");
        io::stdout().flush().unwrap();

        let message = term.read_line().unwrap_or_default();

        let message = match to_encrypted(&message.trim().to_owned(), &contact.contact_key) {
            Ok(message) => message,
            Err(_) => {
                term.clear_screen().unwrap();
                println!("{}", style("Failed to encrypt the message!").red());
                continue;
            }
        };

        let _ = clipboard.set_text(&message);

        print!(
            "
            \r------------------------------------------
            \r{}
            \r------------------------------------------

            \r{}
            \rSend the encrypted message to your contact and press enter to exit when ready...",
            message,
            style("(The message has been copied to your clipboard)").green()
        );
        io::stdout().flush().unwrap();

        let _ = term.read_line().unwrap_or_default();
        return;
    }
}

pub fn decrypt(term: &Term, program_data: &ProgramData) {
    term.clear_screen().unwrap();

    loop {
        print!("Encrypted message (leave empty to exit): ");
        io::stdout().flush().unwrap();

        let message = term.read_line().unwrap_or_default();
        let message = message.trim().to_owned();

        if message.is_empty() {
            return;
        }

        for contact in &program_data.contacts {
            if let Ok(message) = from_encrypted::<String>(&message, &contact.contact_key) {
                print!(
                    "
                    \rFrom: {}
                    \r------------------------------------------
                    \r{}
                    \r------------------------------------------

                    \rPress enter to exit when ready...",
                    contact.contact_name, message
                );
                io::stdout().flush().unwrap();

                let _ = term.read_line().unwrap_or_default();
                return;
            }
        }

        term.clear_screen().unwrap();
        println!("{}", style("Failed to decrypt the message!").red());
        continue;
    }
}
