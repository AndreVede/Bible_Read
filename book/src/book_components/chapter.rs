use super::chapter_number::ChapterNumber;

#[derive(Debug, PartialEq, Clone)]
pub struct Chapter {
    number: ChapterNumber,
    max_verse: u8
}

impl Chapter {
    pub fn new(number: ChapterNumber, max_verse: u8) -> Chapter {
        Chapter {
            number,
            max_verse
        }
    }

    pub fn get_chapter_number(&self) -> &ChapterNumber {
        &self.number
    }

    pub fn get_max_verse(&self) -> &u8 {
        &self.max_verse
    }
}
