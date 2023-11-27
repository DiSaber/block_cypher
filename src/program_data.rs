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

    pub fn format_contacts(&self, include_built_ins: bool) -> String {
        self.contacts
            .iter()
            .enumerate()
            .map(|(i, contact)| {
                if !include_built_ins && contact.contact_name == "Personal (Built-in)" {
                    String::new()
                } else if i < self.contacts.len() - 1 {
                    contact.contact_name.clone() + "|"
                } else {
                    contact.contact_name.clone()
                }
            })
            .collect()
    }
}
