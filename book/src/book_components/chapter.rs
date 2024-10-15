use super::{chapter_number::ChapterNumber, max_verse::MaxVerse};

#[derive(Debug, PartialEq, Clone)]
pub struct Chapter {
    number: ChapterNumber,
    max_verse: MaxVerse,
}

impl Chapter {
    pub fn new(number: ChapterNumber, max_verse: MaxVerse) -> Chapter {
        Chapter { number, max_verse }
    }

    pub fn get_chapter_number(&self) -> &ChapterNumber {
        &self.number
    }

    pub fn get_max_verse(&self) -> &MaxVerse {
        &self.max_verse
    }
}
