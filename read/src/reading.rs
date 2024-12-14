use bible::{bible_enum::BibleEnum, BIBLE};
use book::{
    book_components::{chapter::Chapter, chapter_number::ChapterNumber, verse::Verse},
    Book,
};
use serde::{Deserialize, Serialize};
use strum::IntoEnumIterator;

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ReadingError {
    #[error("This chapter is not listed in the book")]
    ChapterNotInBook,
    #[error("This verse cannot be in this chapter")]
    VerseNotInChapter,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Reading {
    current_book: BibleEnum,
    current_chapter: ChapterNumber,
    current_verse: Verse,
}

impl std::fmt::Display for Reading {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {}:{}",
            BIBLE[self.current_book].name,
            u8::from(self.current_chapter),
            u8::from(self.current_verse)
        )
    }
}

impl Reading {
    pub fn new(
        book: BibleEnum,
        chapter: ChapterNumber,
        verse: Verse,
    ) -> Result<Reading, ReadingError> {
        Self::validate_fields(&book, &chapter, &verse)?;

        Ok(Reading {
            current_book: book,
            current_chapter: chapter,
            current_verse: verse,
        })
    }

    pub fn current_book(&self) -> &BibleEnum {
        &self.current_book
    }

    pub fn current_chapter(&self) -> &ChapterNumber {
        &self.current_chapter
    }

    pub fn current_verse(&self) -> &Verse {
        &self.current_verse
    }

    pub fn modify_reading(
        &mut self,
        book: BibleEnum,
        chapter: ChapterNumber,
        verse: Verse,
    ) -> Result<(), ReadingError> {
        Self::validate_fields(&book, &chapter, &verse)?;

        *self = Reading {
            current_book: book,
            current_chapter: chapter,
            current_verse: verse,
        };

        Ok(())
    }

    pub fn set_current_book(&mut self, book: BibleEnum) -> Result<(), ReadingError> {
        Self::validate_fields(&book, &self.current_chapter, &self.current_verse)?;

        self.current_book = book;

        Ok(())
    }

    pub fn set_current_chapter(&mut self, chapter: ChapterNumber) -> Result<(), ReadingError> {
        Self::validate_fields(&self.current_book, &chapter, &self.current_verse)?;

        self.current_chapter = chapter;

        Ok(())
    }

    pub fn set_current_verse(&mut self, verse: Verse) -> Result<(), ReadingError> {
        let current_book: Book = BIBLE[self.current_book].clone();
        let chapter: &Chapter = current_book.chapters.get(self.current_chapter).unwrap();

        Self::validate_verse(chapter, &verse)?;

        self.current_verse = verse;

        Ok(())
    }

    pub fn next_book(&mut self, count: &u8) -> Result<(), ReadingError> {
        let new_book = match BibleEnum::iter()
            .skip_while(|&value| value < self.current_book)
            .nth(*count as usize)
        {
            Some(book) => book,
            None => BibleEnum::Genesis,
        };
        self.modify_reading(new_book, 1u8.try_into().unwrap(), 1u8.try_into().unwrap())?;

        Ok(())
    }

    pub fn next_chapter(&mut self, count: &u8) -> Result<(), ReadingError> {
        let book: Book = BIBLE[self.current_book].clone();
        let last_chapter: ChapterNumber = *book
            .chapters
            .into_iter()
            .last()
            .unwrap()
            .clone()
            .get_chapter_number();

        if let Some((new_chapter_number, _)) = book
            .chapters
            .range(self.current_chapter..last_chapter)
            .nth(*count as usize)
        {
            self.modify_reading(
                self.current_book,
                *new_chapter_number,
                1u8.try_into().unwrap(),
            )?;
        } else {
            self.next_book(&1u8)?;
        }

        Ok(())
    }

    pub fn next_verse(&mut self, count: &u8) -> Result<(), ReadingError> {
        let book: Book = BIBLE[self.current_book].clone();
        let max_chapter: Verse = *book.chapters[self.current_chapter].get_max_verse();

        if u8::from(self.current_verse) + *count <= u8::from(max_chapter) {
            self.modify_reading(
                self.current_book,
                self.current_chapter,
                (u8::from(self.current_verse) + *count).try_into().unwrap(),
            )?;
        } else {
            self.next_chapter(&1u8)?;
        }

        Ok(())
    }

    pub fn previous_book(&mut self, count: &u8) -> Result<(), ReadingError> {
        let new_book = match BibleEnum::iter()
            .rev()
            .skip_while(|&value| value > self.current_book)
            .nth(*count as usize)
        {
            Some(book) => book,
            None => BibleEnum::Revelation,
        };

        self.modify_reading(new_book, 1u8.try_into().unwrap(), 1u8.try_into().unwrap())?;

        Ok(())
    }

    pub fn previous_chapter(&mut self, count: &u8) -> Result<(), ReadingError> {
        let book: Book = BIBLE[self.current_book].clone();

        if let Some((new_chapter_number, _)) = book
            .chapters
            .range(1u8.try_into().unwrap()..self.current_chapter)
            .nth_back((*count - 1) as usize)
        {
            self.modify_reading(
                self.current_book,
                *new_chapter_number,
                1u8.try_into().unwrap(),
            )?;
        } else {
            self.previous_book(&1u8)?;
        }

        Ok(())
    }

    pub fn previous_verse(&mut self, count: &u8) -> Result<(), ReadingError> {
        let value_verse: i16 = u8::from(self.current_verse) as i16;

        let substract: i16 = value_verse - (*count as i16);

        if substract > 0i16 {
            self.modify_reading(
                self.current_book,
                self.current_chapter,
                (substract as u8).try_into().unwrap(),
            )?;
        } else {
            self.previous_chapter(&1u8)?;
        }

        Ok(())
    }

    fn validate_fields(
        book: &BibleEnum,
        chapter: &ChapterNumber,
        verse: &Verse,
    ) -> Result<(), ReadingError> {
        let current_book: Book = BIBLE[book].clone();

        if let Some(chapter_in_book) = current_book.chapters.get(*chapter) {
            Self::validate_verse(chapter_in_book, verse)
        } else {
            Err(ReadingError::ChapterNotInBook)
        }
    }

    fn validate_verse(chapter: &Chapter, verse: &Verse) -> Result<(), ReadingError> {
        if u8::from(verse) > u8::from(chapter.get_max_verse()) {
            Err(ReadingError::VerseNotInChapter)
        } else {
            Ok(())
        }
    }
}

#[cfg(test)]
mod tests {
    use book::book_components::{chapter_number::ChapterNumber, name::BookName};

    use super::*;

    #[test]
    fn test_create_reading_and_modify() {
        let chapter_number = ChapterNumber::try_from(1u8).unwrap();

        let verse = Verse::try_from(5u8).unwrap();

        let mut reading = Reading::new(
            BibleEnum::Exodus,
            ChapterNumber::try_from(2u8).unwrap(),
            verse,
        )
        .unwrap();

        reading
            .modify_reading(BibleEnum::Exodus, chapter_number, verse)
            .unwrap();

        assert_eq!(reading.current_verse(), &verse);
        assert_eq!(reading.current_chapter(), &chapter_number);
        assert_eq!(
            BIBLE[reading.current_book].name,
            BookName::try_from("Exodus").unwrap()
        );
    }

    #[test]
    fn test_verse_not_in_chapter() {
        let chapter_number = ChapterNumber::try_from(1u8).unwrap();

        let verse = Verse::try_from(111u8).unwrap();

        let error = Reading::new(BibleEnum::Exodus, chapter_number, verse).unwrap_err();

        assert_eq!(error.to_string(), "This verse cannot be in this chapter");
    }

    #[test]
    fn test_chapter_not_in_book() {
        let chapter_number = ChapterNumber::try_from(111u8).unwrap();

        let verse = Verse::try_from(10u8).unwrap();

        let error = Reading::new(BibleEnum::Exodus, chapter_number, verse).unwrap_err();

        assert_eq!(error.to_string(), "This chapter is not listed in the book");
    }

    #[test]
    fn test_display() {
        let chapter_number = ChapterNumber::try_from(1u8).unwrap();

        let verse = Verse::try_from(1u8).unwrap();

        let reading = Reading::new(BibleEnum::Genesis, chapter_number, verse).unwrap();

        assert_eq!(reading.to_string(), "Genesis 1:1");
    }

    #[test]
    fn test_next() {
        todo!();
    }

    #[test]
    fn test_previous() {
        todo!();
    }
}
