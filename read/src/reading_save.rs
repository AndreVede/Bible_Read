use std::io::Write;
use std::{fs::OpenOptions, io::Read};

use serde::{Deserialize, Serialize};

use crate::{Reading, ReadingError};

const NAME_FILE_SAVE: &str = "reading_save.ron";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ReadingSave {
    current_book: String,
    current_chapter: u8,
    current_verse: u8,
}

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ReadingSaveError {}

impl ReadingSave {
    pub fn new() -> ReadingSave {
        if let Ok(reading_save_from_file) = Self::from_save() {
            reading_save_from_file
        } else {
            Default::default()
        }
    }

    fn from_save() -> std::io::Result<ReadingSave> {
        let mut file = OpenOptions::new().read(true).open(NAME_FILE_SAVE)?;
        let mut content_file: String = String::new();

        file.read_to_string(&mut content_file)?;

        let reading_save: ReadingSave = ron::from_str(&content_file)
            .expect("There is a problem with convert file content into ReadingSave");

        Ok(reading_save)
    }

    pub fn to_save(&self) -> std::io::Result<()> {
        let data_to_save = ron::to_string(&self).expect("Problem to convert into ron");

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(NAME_FILE_SAVE)?;

        file.write_all(data_to_save.as_bytes())?;

        Ok(())
    }
}

impl From<Reading> for ReadingSave {
    fn from(value: Reading) -> Self {
        todo!();
    }
}

impl From<&Reading> for ReadingSave {
    fn from(value: &Reading) -> Self {
        todo!();
    }
}

impl TryFrom<ReadingSave> for Reading {
    type Error = ReadingError;

    fn try_from(value: ReadingSave) -> Result<Self, Self::Error> {
        todo!();
    }
}

impl TryFrom<&ReadingSave> for Reading {
    type Error = ReadingError;

    fn try_from(value: &ReadingSave) -> Result<Self, Self::Error> {
        todo!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_reading() {
        todo!();
    }

    #[test]
    fn test_from_ref_reading() {
        todo!();
    }

    #[test]
    fn test_try_from_reading_save() {
        todo!();
    }

    #[test]
    fn test_try_from_ref_reading_save() {
        todo!();
    }

    #[test]
    fn test_get_from_save() {
        todo!();
    }

    #[test]
    fn test_to_save() {
        todo!();
    }
}
