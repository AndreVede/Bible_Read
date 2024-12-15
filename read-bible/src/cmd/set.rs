use bible::bible_enum::BibleEnum;
use book::book_components::{chapter_number::ChapterNumber, verse::Verse};
use clap::Args;
use read::launch_reading;
use read::reading::Reading;
use std::str::FromStr;

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct SetArgs {
    /// The book to set
    #[arg(long)]
    pub book: Option<String>,
    /// The chapter to set
    #[arg(long)]
    pub chapter: Option<u8>,
    /// The verse to set
    #[arg(long)]
    pub verse: Option<u8>,
}

#[derive(Debug, thiserror::Error)]
pub enum SetError {
    #[error("The setting ordered is invalid. Book, chapter or verse is invalid.")]
    ReadingInvalid,
}

pub fn main(path: std::path::PathBuf, args: &SetArgs) -> anyhow::Result<()> {
    let client = launch_reading(1, path);

    let reading: Result<Reading, SetError> = match client.get_reading_from_file() {
        Ok(existant_reading) => {
            // If a reading is getted by a file
            // Get Book if there is
            let book: BibleEnum = match &args.book {
                Some(book_arg) => match BibleEnum::from_str(book_arg) {
                    Ok(book_enum) => book_enum,
                    Err(_) => *existant_reading.current_book(),
                },
                None => *existant_reading.current_book(),
            };
            // Get Chapter if there is
            let chapter: ChapterNumber = match args.chapter {
                Some(number) => ChapterNumber::try_from(number)
                    .unwrap_or_else(|_| *existant_reading.current_chapter()),
                None => *existant_reading.current_chapter(),
            };
            // Get Verse if there is
            let verse: Verse = match args.verse {
                Some(number) => {
                    Verse::try_from(number).unwrap_or_else(|_| *existant_reading.current_verse())
                }
                None => *existant_reading.current_verse(),
            };

            let mut new_reading = existant_reading.clone();

            new_reading
                .modify_reading(book, chapter, verse)
                .map_err(|_| SetError::ReadingInvalid)?;

            Ok(new_reading)
        }
        Err(_) => {
            // If there is no reading saved, create it
            let book = match &args.book {
                Some(book_arg) => {
                    if let Ok(book_enum) = BibleEnum::from_str(book_arg) {
                        book_enum
                    } else {
                        BibleEnum::Genesis
                    }
                }
                None => BibleEnum::Genesis,
            };

            let chapter = match args.chapter {
                Some(chapter_arg) => ChapterNumber::try_from(chapter_arg).unwrap(),
                None => ChapterNumber::try_from(1u8).unwrap(),
            };

            let verse = match args.verse {
                Some(verse_arg) => Verse::try_from(verse_arg).unwrap(),
                None => Verse::try_from(1u8).unwrap(),
            };

            match Reading::new(book, chapter, verse) {
                Ok(reading) => Ok(reading),
                Err(_) => Err(SetError::ReadingInvalid),
            }
        }
    };

    client.set_current_reading(reading.unwrap())?;

    client.save_reading_in_file()?;

    Ok(())
}
