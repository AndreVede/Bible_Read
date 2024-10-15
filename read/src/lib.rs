use book::{
    book_components::{chapter::Chapter, verse::Verse},
    Book,
};

#[derive(Debug, Clone, Default)]
pub struct Reading {
    current_book: Option<Book>,
    current_chapter: Option<Chapter>,
    current_verse: Option<Verse>,
}

impl Reading {
    pub fn new() -> Reading {
        Default::default()
    }

    pub fn current_book(&self) -> Option<&Book> {
        self.current_book.as_ref()
    }

    pub fn current_chapter(&self) -> Option<&Chapter> {
        self.current_chapter.as_ref()
    }

    pub fn current_verse(&self) -> Option<&Verse> {
        self.current_verse.as_ref()
    }

    pub fn set_current_book(&mut self, book: Book) {
        self.current_book = Some(book);
    }

    pub fn set_current_chapter(&mut self, chapter: Chapter) {
        self.current_chapter = Some(chapter);
    }

    pub fn set_current_verse(&mut self, verse: Verse) {
        self.current_verse = Some(verse);
    }
}
