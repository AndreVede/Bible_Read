use super::{chapter_number::ChapterNumber, verse::Verse};

#[derive(Debug, PartialEq, Clone)]
pub struct Chapter {
    number: ChapterNumber,
    verse: Verse,
}

impl Chapter {
    pub fn new(number: ChapterNumber, verse: Verse) -> Chapter {
        Chapter { number, verse }
    }

    pub fn get_chapter_number(&self) -> &ChapterNumber {
        &self.number
    }

    pub fn get_verse(&self) -> &Verse {
        &self.verse
    }
}
