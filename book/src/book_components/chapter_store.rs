use std::{
    collections::BTreeMap,
    ops::{Index, IndexMut},
};

use super::{chapter::Chapter, chapter_number::ChapterNumber};

#[derive(Debug, PartialEq, Clone, Default)]
pub struct ChapterStore {
    store: BTreeMap<ChapterNumber, Chapter>,
}

impl ChapterStore {
    pub fn new() -> ChapterStore {
        ChapterStore {
            store: BTreeMap::new(),
        }
    }

    pub fn add_chapter(&mut self, chapter: Chapter) {
        self.store.insert(*chapter.get_chapter_number(), chapter);
    }

    pub fn get(&self, chapter_number: ChapterNumber) -> Option<&Chapter> {
        self.store.get(&chapter_number)
    }

    pub fn get_mut(&mut self, chapter_number: ChapterNumber) -> Option<&mut Chapter> {
        self.store.get_mut(&chapter_number)
    }
}

impl Index<ChapterNumber> for ChapterStore {
    type Output = Chapter;

    fn index(&self, index: ChapterNumber) -> &Self::Output {
        self.store.get(&index).unwrap()
    }
}

impl Index<&ChapterNumber> for ChapterStore {
    type Output = Chapter;

    fn index(&self, index: &ChapterNumber) -> &Self::Output {
        &self[*index]
    }
}

impl IndexMut<ChapterNumber> for ChapterStore {
    fn index_mut(&mut self, index: ChapterNumber) -> &mut Self::Output {
        self.store.get_mut(&index).unwrap()
    }
}

impl IndexMut<&ChapterNumber> for ChapterStore {
    fn index_mut(&mut self, index: &ChapterNumber) -> &mut Self::Output {
        &mut self[*index]
    }
}

impl<'a> IntoIterator for &'a ChapterStore {
    type Item = &'a Chapter;
    type IntoIter = std::collections::btree_map::Values<'a, ChapterNumber, Chapter>;

    fn into_iter(self) -> Self::IntoIter {
        self.store.values()
    }
}

#[cfg(test)]
mod tests {
    use crate::book_components::verse::Verse;

    use super::*;

    #[test]
    fn test_work() {
        let mut store: ChapterStore = ChapterStore::new();

        let n_chapters: u8 = 30;

        for i in 1..n_chapters {
            let chapter_number: ChapterNumber = i.try_into().unwrap();
            let chapter: Chapter = Chapter::new(chapter_number, Verse::try_from(40u8).unwrap());

            store.add_chapter(chapter);
        }

        let mut num: u8 = 1;
        for chapter in store.into_iter() {
            let chapter_num: &ChapterNumber = chapter.get_chapter_number();
            assert_eq!(u8::from(chapter_num), num);
            num += 1;
        }
    }
}
