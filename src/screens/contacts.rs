use base64::{engine::general_purpose, Engine};
use oqs::*;
use std::io::{self, Write};

use arboard::Clipboard;
use console::{style, Key, Term};

use crate::{contact::Contact, program_data::ProgramData, utils::save_config};

pub fn contacts(term: &Term, clipboard: &mut Clipboard, program_data: &mut ProgramData) {
    loop {
        term.clear_screen().unwrap();
        println!(
            "{}
            \r{} {}

            \rPress the {} key to add a contact
            \rPress the {} key to edit an existing contact

            \rPress the {} key to return to the main menu",
            program_data.format_contacts(),
            program_data.contacts.len(),
            if program_data.contacts.len() == 1 {
                "contact"
            } else {
                "contacts"
            },
            style("'a'").cyan(),
            style("'e'").cyan(),
            style("escape").red()
        );

        let key = term.read_key().unwrap_or(Key::Unknown);

        match key {
            Key::Char('a') => key_exchange(term, clipboard, program_data),
            Key::Char('e') => edit_contact(term, program_data),
            Key::Escape => {
                return;
            }
            _ => (),
        };
    }
}

fn key_exchange(term: &Term, clipboard: &mut Clipboard, program_data: &mut ProgramData) {
    term.clear_screen().unwrap();

    loop {
        print!(
            "{}
            \r{} {}

            \rNew contact name (leave empty to exit): ",
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

        if program_data
            .contacts
            .iter()
            .any(|contact| contact.contact_name == contact_name)
        {
            term.clear_screen().unwrap();
            println!("Contact already exists!");
            continue;
        }

        println!(
            "\nPress the {} key to start a key exchange
            \rPress the {} key to enter a receiving key

            \rPress the {} key to exit",
            style("'s'").cyan(),
            style("'r'").cyan(),
            style("escape").red()
        );

        let key = term.read_key().unwrap_or(Key::Unknown);

        match key {
            Key::Char('s') => {
                let kyber = kem::Kem::new(kem::Algorithm::Kyber1024).unwrap();
                let (public_key, secret_key) = kyber.keypair().unwrap();
                let encoded_public_key = general_purpose::STANDARD_NO_PAD.encode(public_key);

                let _ = clipboard.set_text(&encoded_public_key);

                println!(
                    "
                    \r------------------------------------------
                    \r{}
                    \r------------------------------------------
                    
                    \r{}
                    \rSend this receiving key to the other user and enter the cipher text they send back to you
                    
                    ", 
                    encoded_public_key,
                    style("(The receiving key has been copied to your clipboard)").green()
                );

                print!("Cipher Text: ");

                io::stdout().flush().unwrap();
                let buffer = term.read_line().unwrap_or_default();

                let cipher_text = match general_purpose::STANDARD_NO_PAD.decode(buffer.trim_end()) {
                    Ok(cipher_text) => cipher_text,
                    Err(_) => {
                        term.clear_screen().unwrap();
                        println!("Invalid cipher text!");
                        continue;
                    }
                };

                let cipher_text = match kyber.ciphertext_from_bytes(cipher_text.as_slice()) {
                    Some(cipher_text) => cipher_text,
                    None => {
                        term.clear_screen().unwrap();
                        println!("Invalid cipher text!");
                        continue;
                    }
                };
                let shared_secret = kyber.decapsulate(&secret_key, &cipher_text).unwrap();

                program_data.contacts.push(Contact {
                    contact_name,
                    contact_key: shared_secret.into_vec().as_slice().try_into().unwrap(),
                });
                save_config(program_data, &program_data.hashed_password);

                return;
            }
            Key::Char('r') => {
                let kyber = kem::Kem::new(kem::Algorithm::Kyber1024).unwrap();

                print!("\nReceiving key: ");

                io::stdout().flush().unwrap();
                let buffer = term.read_line().unwrap_or_default();

                let public_key = match general_purpose::STANDARD_NO_PAD.decode(buffer.trim_end()) {
                    Ok(public_key) => public_key,
                    Err(_) => {
                        term.clear_screen().unwrap();
                        println!("Invalid receiving key!");
                        continue;
                    }
                };
                let public_key = kyber.public_key_from_bytes(public_key.as_slice()).unwrap();

                let (cipher_text, shared_secret) = kyber.encapsulate(public_key).unwrap();
                let encoded_cipher_text = general_purpose::STANDARD_NO_PAD.encode(cipher_text);

                let _ = clipboard.set_text(&encoded_cipher_text);

                println!(
                    "
                    \r------------------------------------------
                    \r{}
                    \r------------------------------------------
                    
                    \r{}
                    \rSend this cipher text back to the other user

                    \rPress enter to save the contact when ready...",
                    encoded_cipher_text,
                    style("(The cipher text has been copied to your clipboard)").green()
                );

                let _ = term.read_line().unwrap_or_default();

                program_data.contacts.push(Contact {
                    contact_name,
                    contact_key: shared_secret.into_vec().as_slice().try_into().unwrap(),
                });
                save_config(program_data, &program_data.hashed_password);

                return;
            }
            Key::Escape => return,
            _ => {
                term.clear_screen().unwrap();
                println!("Invalid option!");
                continue;
            }
        };
    }
}

fn edit_contact(term: &Term, program_data: &mut ProgramData) {
    term.clear_screen().unwrap();

    loop {
        print!(
            "{}
            \r{} {}

            \rContact to edit (leave empty to exit): ",
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

        let contact_index = match program_data
            .contacts
            .iter()
            .position(|contact| contact.contact_name == contact_name)
        {
            Some(contact_index) => contact_index,
            None => {
                term.clear_screen().unwrap();
                println!("Could not find the contact!");
                continue;
            }
        };

        println!(
            "\nPress the {} key to edit the contact's name
            \rPress the {} key to delete the contact

            \rPress the {} key to exit\n",
            style("'e'").cyan(),
            style("'d'").cyan(),
            style("escape").red()
        );

        let key = term.read_key().unwrap_or(Key::Unknown);

        match key {
            Key::Char('e') => {
                print!("Contact name: ");
                io::stdout().flush().unwrap();

                let contact_name = term
                    .read_line_initial_text(&contact_name)
                    .unwrap_or_default();

                if contact_name.is_empty() {
                    term.clear_screen().unwrap();
                    println!("Contact name cannot be empty!");
                    continue;
                }

                program_data.contacts[contact_index].contact_name = contact_name;
                save_config(program_data, &program_data.hashed_password);
            }
            Key::Char('d') => {
                program_data.contacts.remove(contact_index);
                save_config(program_data, &program_data.hashed_password);
                return;
            }
            Key::Escape => return,
            _ => {
                term.clear_screen().unwrap();
                println!("Invalid option!");
                continue;
            }
        };
    }
}
