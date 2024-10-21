use serde::{Deserialize, Serialize};

use super::{chapter_number::ChapterNumber, verse::Verse};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Chapter {
    number: ChapterNumber,
    max_verse: Verse,
}

impl Chapter {
    pub fn new(number: ChapterNumber, max_verse: Verse) -> Chapter {
        Chapter { number, max_verse }
    }

    pub fn get_chapter_number(&self) -> &ChapterNumber {
        &self.number
    }

    pub fn get_max_verse(&self) -> &Verse {
        &self.max_verse
    }
}
