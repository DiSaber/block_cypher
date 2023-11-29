use serde::{Deserialize, Serialize};

use crate::{contact::Contact, encryption_handler::to_encrypted};
use directories::ProjectDirs;
use std::{fs, path::Path};

pub const VERSION_CODE: &str = "v3.2.0";

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

    pub fn save_config(&self) {
        let data_path = ProjectDirs::from("com", "DiSaber", "BlockCypher").unwrap();
        let data_path: &Path = data_path.config_dir();
        let data_file = data_path.join("block_cypher.data");

        if !data_path.exists() {
            fs::create_dir_all(data_path).unwrap();
        }

        fs::write(
            data_file,
            to_encrypted(self, &self.hashed_password)
                .unwrap()
                .to_binary(),
        )
        .unwrap();
    }
}
