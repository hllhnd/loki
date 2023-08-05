use std::error::Error;
use std::fs;
use std::path::PathBuf;

use crate::executable::Executable;

pub struct CreateDirectory {
    pub directory: PathBuf,
}

impl Executable for CreateDirectory {
    fn execute(&mut self) -> Result<i32, Box<dyn Error + Send + Sync>> {
        Ok(fs::create_dir_all(&self.directory).map(|_| 0)?)
    }
}
