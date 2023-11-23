use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct FileContainer {
    pub file: Vec<u8>,
    pub filename: String,
}
