mod reading_save;

use book::{
    book_components::{chapter_number::ChapterNumber, verse::Verse},
    Book,
};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ReadingError {
    #[error("This chapter is not listed in the book")]
    ChapterNotInBook,
    #[error("This verse cannot be in this chapter")]
    VerseNotInChapter,
}

#[derive(Debug, Clone, Default)]
pub struct Reading {
    current_book: Option<Book>,
    current_chapter: Option<ChapterNumber>,
    current_verse: Option<Verse>,
}

impl Reading {
    pub fn new() -> Reading {
        Default::default()
    }

    pub fn current_book(&self) -> Option<&Book> {
        self.current_book.as_ref()
    }

    pub fn current_chapter(&self) -> Option<&ChapterNumber> {
        self.current_chapter.as_ref()
    }

    pub fn current_verse(&self) -> Option<&Verse> {
        self.current_verse.as_ref()
    }

    pub fn modify_reading(
        &mut self,
        book: Book,
        chapter: ChapterNumber,
        verse: Verse,
    ) -> Result<(), ReadingError> {
        Self::validate_fields(&book, &chapter, &verse)?;

        *self = Reading {
            current_book: Some(book),
            current_chapter: Some(chapter),
            current_verse: Some(verse),
        };

        Ok(())
    }

    pub fn validate_fields(
        book: &Book,
        chapter: &ChapterNumber,
        verse: &Verse,
    ) -> Result<(), ReadingError> {
        if let Some(chapter_in_book) = book.chapters.get(*chapter) {
            if u8::from(verse) > u8::from(chapter_in_book.get_max_verse()) {
                Err(ReadingError::VerseNotInChapter)
            } else {
                Ok(())
            }
        } else {
            Err(ReadingError::ChapterNotInBook)
        }
    }
}

#[cfg(test)]
mod tests {
    use book::book_components::{
        chapter::Chapter, chapter_number::ChapterNumber, chapter_store::ChapterStore,
        name::BookName,
    };

    use super::*;

    #[test]
    fn test_create_reading_and_modify() {
        let mut reading = Reading::new();

        let mut book: Book = Book {
            name: "a book".try_into().unwrap(),
            chapters: ChapterStore::new(),
        };

        let chapter_number = ChapterNumber::try_from(1u8).unwrap();

        let chapter = Chapter::new(chapter_number, Verse::try_from(10u8).unwrap());

        book.chapters.add_chapter(chapter);

        let verse = Verse::try_from(5u8).unwrap();

        let _ = reading.modify_reading(book, chapter_number, verse);

        assert_eq!(reading.current_verse().unwrap(), &verse);
        assert_eq!(reading.current_chapter().unwrap(), &chapter_number);
        assert_eq!(
            reading.current_book().unwrap().name,
            BookName::try_from("a book").unwrap()
        );
    }

    #[test]
    fn test_verse_not_in_chapter() {
        let mut reading = Reading::new();

        let mut book: Book = Book {
            name: "a book".try_into().unwrap(),
            chapters: ChapterStore::new(),
        };

        let chapter_number = ChapterNumber::try_from(1u8).unwrap();

        let chapter = Chapter::new(chapter_number, Verse::try_from(10u8).unwrap());

        book.chapters.add_chapter(chapter);

        let verse = Verse::try_from(11u8).unwrap();

        let error = reading
            .modify_reading(book, chapter_number, verse)
            .unwrap_err();

        assert_eq!(error.to_string(), "This verse cannot be in this chapter");
    }

    #[test]
    fn test_chapter_not_in_book() {
        let mut reading = Reading::new();

        let mut book: Book = Book {
            name: "a book".try_into().unwrap(),
            chapters: ChapterStore::new(),
        };

        let chapter_number = ChapterNumber::try_from(1u8).unwrap();

        let chapter = Chapter::new(
            ChapterNumber::try_from(2u8).unwrap(),
            Verse::try_from(10u8).unwrap(),
        );

        book.chapters.add_chapter(chapter);

        let verse = Verse::try_from(10u8).unwrap();

        let error = reading
            .modify_reading(book, chapter_number, verse)
            .unwrap_err();

        assert_eq!(error.to_string(), "This chapter is not listed in the book");
    }
}
