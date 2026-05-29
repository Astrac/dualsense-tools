use std::{error::Error, fmt::Display};

use crate::feeder::backend::BackendId;

#[derive(Debug)]
pub struct BackendError {
    pub backend_id: BackendId,
    pub description: String,
}

impl BackendError {
    pub fn new(backend_id: BackendId, description: String) -> BackendError {
        BackendError {
            backend_id,
            description,
        }
    }
}

impl Display for BackendError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        f.write_str(format!("Backend `{}` error: {}", self.backend_id, self.description).as_str())
    }
}

impl Error for BackendError {}
