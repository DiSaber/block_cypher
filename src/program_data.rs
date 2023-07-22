use serde::{Deserialize, Serialize};

use crate::contact::Contact;

#[derive(Serialize, Deserialize)]
pub struct ProgramData {
    pub hashed_password: [u8; 32],
    pub contacts: Vec<Contact>,
}

impl ProgramData {
    pub fn new(password: &[u8; 32]) -> Self {
        ProgramData {
            hashed_password: *password,
            contacts: Vec::new(),
        }
    }

    pub fn format_contacts(&self) -> String {
        let formatted_contacts: String = self
            .contacts
            .iter()
            .enumerate()
            .map(|(i, contact)| {
                if i < self.contacts.len() - 1 {
                    contact.contact_name.clone() + ", "
                } else {
                    contact.contact_name.clone()
                }
            })
            .collect();

        String::from("[") + &formatted_contacts + "]"
    }
}
