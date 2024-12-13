use std::{
    fs::OpenOptions,
    io::{Read, Write},
    sync::Arc,
};

use crate::reading::Reading;

pub fn get_reading_in_file(path: Arc<std::path::PathBuf>) -> std::io::Result<Option<Reading>> {
    let mut file = OpenOptions::new().read(true).open(&*path)?;

    let mut read_file = String::new();

    file.read_to_string(&mut read_file)?;

    if let Ok(reading) = ron::from_str(&read_file) {
        return Ok(Some(reading));
    }

    Ok(None)
}

pub fn save_reading_in_file(
    path: Arc<std::path::PathBuf>,
    reading: &Reading,
) -> std::io::Result<()> {
    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .truncate(true)
        .open(&*path)?;

    let data = ron::to_string(reading).unwrap();

    file.write_all(data.as_bytes())?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use bible::bible_enum::BibleEnum;
    use book::book_components::{chapter_number::ChapterNumber, verse::Verse};

    use super::*;

    #[test]
    fn test_work() {
        let path: Arc<std::path::PathBuf> = Arc::new("test.ron".into());

        let verse = Verse::try_from(2u8).unwrap();
        let chapter_number = ChapterNumber::try_from(1u8).unwrap();
        let reading: Reading = Reading::new(BibleEnum::Genesis, chapter_number, verse).unwrap();

        save_reading_in_file(path.clone(), &reading).unwrap();

        let result_reading_file: Option<Reading> = get_reading_in_file(path).unwrap();
        assert_eq!(result_reading_file, Some(reading));
    }
}
