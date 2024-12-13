use bible::{bible_enum::BibleEnum, BIBLE};
use book::{
    book_components::{chapter_number::ChapterNumber, verse::Verse},
    Book,
};
use clap::Args;
use read::{launch_reading, reading::Reading};
use strum::IntoEnumIterator;

use super::choice_entity::ChoiceEntity;

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct NextArgs {
    #[arg(short, long, default_value_t, value_enum)]
    pub entity: ChoiceEntity,
    #[arg(short, long, default_value = "1")]
    pub count: u8,
}

fn next_book(reading: &Reading, count: &u8) -> Reading {
    let new_book = match BibleEnum::iter()
        .skip_while(|&value| value < *reading.current_book())
        .nth(*count as usize)
    {
        Some(book) => book,
        None => BibleEnum::Genesis,
    };
    Reading::new(new_book, 1u8.try_into().unwrap(), 1u8.try_into().unwrap()).unwrap()
}

fn next_chapter(reading: &Reading, count: &u8) -> Reading {
    let book: Book = BIBLE[*reading.current_book()].clone();
    let last_chapter: ChapterNumber = *book
        .chapters
        .into_iter()
        .last()
        .unwrap()
        .clone()
        .get_chapter_number();

    if let Some((new_chapter_number, _)) = book
        .chapters
        .range(*reading.current_chapter()..last_chapter)
        .nth(*count as usize)
    {
        Reading::new(
            *reading.current_book(),
            *new_chapter_number,
            1u8.try_into().unwrap(),
        )
        .unwrap()
    } else {
        next_book(reading, &1u8)
    }
}

fn next_verse(reading: &Reading, count: &u8) -> Reading {
    let book: Book = BIBLE[reading.current_book()].clone();
    let max_chapter: Verse = *book.chapters[reading.current_chapter()].get_max_verse();

    if u8::from(reading.current_verse()) + *count <= u8::from(max_chapter) {
        Reading::new(
            *reading.current_book(),
            *reading.current_chapter(),
            (u8::from(reading.current_verse()) + *count)
                .try_into()
                .unwrap(),
        )
        .unwrap()
    } else {
        next_chapter(reading, &1u8)
    }
}

pub fn main(path: std::path::PathBuf, args: &NextArgs) -> anyhow::Result<()> {
    let client = launch_reading(1, path);

    let mut new_reading: Reading = Reading::new(
        BibleEnum::Genesis,
        1u8.try_into().unwrap(),
        1u8.try_into().unwrap(),
    )
    .unwrap();

    if let Ok(existant_reading) = client.get_reading_from_file() {
        match args.entity {
            ChoiceEntity::Book => {
                new_reading = next_book(&existant_reading, &args.count);
            }
            ChoiceEntity::Chapter => {
                new_reading = next_chapter(&existant_reading, &args.count);
            }
            ChoiceEntity::Verse => {
                new_reading = next_verse(&existant_reading, &args.count);
            }
        };
    }

    client
        .set_current_reading(new_reading)
        .expect("There was a problem when we set the next reading");
    client.save_reading_in_file()?;

    Ok(())
}
