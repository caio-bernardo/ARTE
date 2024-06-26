use std::fs;

use crate::term::Result;

#[derive(Default)]
pub struct Buffer {
    pub lines: Vec<String>,
}

impl Buffer {
    pub fn load(file_name: &std::path::Path) -> Result<Self> {
        let contents = fs::read_to_string(file_name)?;
        let lines = contents
            .lines()
            .map(std::string::ToString::to_string)
            .collect();
        Ok(Self { lines })
    }

    pub fn is_empty(&self) -> bool {
        self.lines.is_empty()
    }
}
