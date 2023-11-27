use directories::ProjectDirs;
use std::{fs, path::Path};

use crate::{encryption_handler::to_encrypted, program_data::ProgramData};

pub const VERSION_CODE: &str = "v3.1.0";

pub fn save_config(program_data: &ProgramData, password: &[u8; 32]) {
    let data_path = ProjectDirs::from("com", "DiSaber", "BlockCypher").unwrap();
    let data_path: &Path = data_path.config_dir();
    let data_file = data_path.join("block_cypher.dat");

    if !data_path.exists() {
        fs::create_dir_all(data_path).unwrap();
    }
    fs::write(
        data_file,
        to_encrypted(program_data, password).unwrap().to_binary(),
    )
    .unwrap();
}
