use bible::{bible_enum::BibleEnum, BIBLE};
use book::{
    book_components::{chapter_number::ChapterNumber, verse::Verse},
    Book,
};
use clap::Args;
use read::{launch_reading, reading::Reading};
use strum::IntoEnumIterator;

use super::choice_entity::ChoiceEntity;

pub enum Direction {
    Next,
    Previous,
}

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct NextPreviousArgs {
    #[arg(short, long, default_value_t, value_enum)]
    entity: ChoiceEntity,
    #[arg(short, long, default_value = "1")]
    count: u8,
}

impl NextPreviousArgs {
    fn next_book(&self, reading: &Reading) -> Reading {
        let new_book = match BibleEnum::iter()
            .skip_while(|&value| value < *reading.current_book())
            .nth(self.count as usize)
        {
            Some(book) => book,
            None => BibleEnum::Genesis,
        };
        Reading::new(new_book, 1u8.try_into().unwrap(), 1u8.try_into().unwrap()).unwrap()
    }

    fn next_chapter(&self, reading: &Reading) -> Reading {
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
            .nth(self.count as usize)
        {
            Reading::new(
                *reading.current_book(),
                *new_chapter_number,
                1u8.try_into().unwrap(),
            )
            .unwrap()
        } else {
            self.next_book(reading)
        }
    }

    fn next_verse(&self, reading: &Reading) -> Reading {
        let book: Book = BIBLE[reading.current_book()].clone();
        let max_chapter: Verse = *book.chapters[reading.current_chapter()].get_max_verse();

        if u8::from(reading.current_verse()) + self.count <= u8::from(max_chapter) {
            Reading::new(
                *reading.current_book(),
                *reading.current_chapter(),
                (u8::from(reading.current_verse()) + self.count)
                    .try_into()
                    .unwrap(),
            )
            .unwrap()
        } else {
            self.next_chapter(reading)
        }
    }

    fn previous_book(&self, reading: &Reading) -> Reading {
        todo!()
    }

    fn previous_chapter(&self, reading: &Reading) -> Reading {
        todo!()
    }

    fn previous_verse(&self, reading: &Reading) -> Reading {
        todo!()
    }

    pub fn next(&self, reading: &Reading) -> Reading {
        match self.entity {
            ChoiceEntity::Book => self.next_book(reading),
            ChoiceEntity::Chapter => self.next_chapter(reading),
            ChoiceEntity::Verse => self.next_verse(reading),
        }
    }

    pub fn previous(&self, reading: &Reading) -> Reading {
        match self.entity {
            ChoiceEntity::Book => self.previous_book(reading),
            ChoiceEntity::Chapter => self.previous_chapter(reading),
            ChoiceEntity::Verse => self.previous_verse(reading),
        }
    }
}
pub fn main(
    path: std::path::PathBuf,
    args: &NextPreviousArgs,
    direction: Direction,
) -> anyhow::Result<()> {
    let client = launch_reading(1, path);

    let mut new_reading: Reading = Reading::new(
        BibleEnum::Genesis,
        1u8.try_into().unwrap(),
        1u8.try_into().unwrap(),
    )
    .unwrap();

    if let Ok(existant_reading) = client.get_reading_from_file() {
        new_reading = match direction {
            Direction::Next => args.next(&existant_reading),
            Direction::Previous => args.previous(&existant_reading),
        }
    }

    client
        .set_current_reading(new_reading)
        .expect("There was a problem when we set the new reading");
    client.save_reading_in_file()?;

    Ok(())
}
