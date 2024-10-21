use book::{
    book_components::{chapter::Chapter, chapter_number::ChapterNumber, verse::Verse},
    Book,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ReadingError {
    #[error("This chapter is not listed in the book")]
    ChapterNotInBook,
    #[error("This verse cannot be in this chapter")]
    VerseNotInChapter,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reading {
    current_book: Book,
    current_chapter: ChapterNumber,
    current_verse: Verse,
}

impl Reading {
    pub fn new(book: Book, chapter: ChapterNumber, verse: Verse) -> Result<Reading, ReadingError> {
        Self::validate_fields(&book, &chapter, &verse)?;

        Ok(Reading {
            current_book: book,
            current_chapter: chapter,
            current_verse: verse,
        })
    }

    pub fn current_book(&self) -> &Book {
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
        book: Book,
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

    pub fn set_current_book(&mut self, book: Book) -> Result<(), ReadingError> {
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
        let chapter: &Chapter = self
            .current_book
            .chapters
            .get(self.current_chapter)
            .unwrap();

        Self::validate_verse(chapter, &verse)?;

        self.current_verse = verse;

        Ok(())
    }

    fn validate_fields(
        book: &Book,
        chapter: &ChapterNumber,
        verse: &Verse,
    ) -> Result<(), ReadingError> {
        if let Some(chapter_in_book) = book.chapters.get(*chapter) {
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
    use book::book_components::{
        chapter::Chapter, chapter_number::ChapterNumber, chapter_store::ChapterStore,
        name::BookName,
    };

    use super::*;

    #[test]
    fn test_create_reading_and_modify() {
        let mut book: Book = Book {
            name: "a book".try_into().unwrap(),
            chapters: ChapterStore::new(),
        };

        let chapter_number = ChapterNumber::try_from(1u8).unwrap();

        let chapter = Chapter::new(chapter_number, Verse::try_from(10u8).unwrap());

        book.chapters.add_chapter(chapter);

        // add another chapter to test the init
        book.chapters.add_chapter(Chapter::new(
            ChapterNumber::try_from(2u8).unwrap(),
            Verse::try_from(6u8).unwrap(),
        ));

        let verse = Verse::try_from(5u8).unwrap();

        let mut reading = Reading::new(
            book.clone(),
            ChapterNumber::try_from(2u8).unwrap(),
            verse.clone(),
        )
        .unwrap();

        reading.modify_reading(book, chapter_number, verse).unwrap();

        assert_eq!(reading.current_verse(), &verse);
        assert_eq!(reading.current_chapter(), &chapter_number);
        assert_eq!(
            reading.current_book().name,
            BookName::try_from("a book").unwrap()
        );
    }

    #[test]
    fn test_verse_not_in_chapter() {
        let mut book: Book = Book {
            name: "a book".try_into().unwrap(),
            chapters: ChapterStore::new(),
        };

        let chapter_number = ChapterNumber::try_from(1u8).unwrap();

        let chapter = Chapter::new(chapter_number, Verse::try_from(10u8).unwrap());

        book.chapters.add_chapter(chapter);

        let verse = Verse::try_from(11u8).unwrap();

        let error = Reading::new(book, chapter_number, verse).unwrap_err();

        assert_eq!(error.to_string(), "This verse cannot be in this chapter");
    }

    #[test]
    fn test_chapter_not_in_book() {
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

        let error = Reading::new(book, chapter_number, verse).unwrap_err();

        assert_eq!(error.to_string(), "This chapter is not listed in the book");
    }
}
