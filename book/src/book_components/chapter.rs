use super::chapter_number::ChapterNumber;

#[derive(Debug, PartialEq, Clone)]
pub struct Chapter {
    number: ChapterNumber,
    current_verse: u8,
    max_verse: u8
}

impl Chapter {
    pub fn new(number: ChapterNumber, max_verse: u8) -> Chapter {
        Chapter {
            number: number,
            current_verse: 0,
            max_verse: max_verse
        }
    }

    pub fn get_chapter_number(&self) -> &ChapterNumber {
        &self.number
    }

    pub fn get_current_verse(&self) -> &u8 {
        &self.current_verse
    }

    pub fn set_current_verse(&mut self, new_current_verse: u8) {
        self.current_verse = new_current_verse;
    }
}
