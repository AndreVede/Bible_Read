use book::Book;
use init_bible::init_books;
use lazy_static::lazy_static;
use std::{
    collections::btree_map::BTreeMap,
    ops::{Index, IndexMut},
};
use strum::IntoEnumIterator;

pub mod bible_enum;
mod constants;
mod init_bible;

use bible_enum::BibleEnum;

#[derive(Debug)]
pub struct Bible {
    books: BTreeMap<BibleEnum, Book>,
}

impl Default for Bible {
    fn default() -> Self {
        let mut books = BTreeMap::new();
        let data: Vec<Book> = init_books();

        for (book, enum_value) in data.iter().zip(BibleEnum::iter()) {
            books.insert(enum_value, book.clone());
        }

        Bible { books }
    }
}

impl Bible {
    pub fn new() -> Bible {
        Default::default()
    }

    pub fn range(
        &self,
        range: core::ops::Range<BibleEnum>,
    ) -> std::collections::btree_map::Range<BibleEnum, Book> {
        self.books.range(range)
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BibleError {
    #[error("Book not found in Bible")]
    BookNotFound,
}

impl Index<BibleEnum> for Bible {
    type Output = Book;

    fn index(&self, index: BibleEnum) -> &Self::Output {
        self.books.get(&index).unwrap()
    }
}

impl Index<&BibleEnum> for Bible {
    type Output = Book;

    fn index(&self, index: &BibleEnum) -> &Self::Output {
        &self[*index]
    }
}

impl IndexMut<BibleEnum> for Bible {
    fn index_mut(&mut self, index: BibleEnum) -> &mut Self::Output {
        self.books.get_mut(&index).unwrap()
    }
}

impl IndexMut<&BibleEnum> for Bible {
    fn index_mut(&mut self, index: &BibleEnum) -> &mut Self::Output {
        &mut self[*index]
    }
}

impl<'a> IntoIterator for &'a Bible {
    type Item = &'a Book;
    type IntoIter = std::collections::btree_map::Values<'a, BibleEnum, Book>;

    fn into_iter(self) -> Self::IntoIter {
        self.books.values()
    }
}

lazy_static! {
    pub static ref BIBLE: Bible = Bible::new();
}

#[cfg(test)]
mod tests {
    use book::book_components::{chapter_number::ChapterNumber, verse::Verse};

    use super::*;

    #[test]
    fn test_work() {
        let bible: Bible = Bible::new();

        assert_eq!(
            bible[BibleEnum::Genesis]
                .chapters
                .get(ChapterNumber::try_from(1u8).unwrap())
                .unwrap()
                .get_max_verse(),
            &Verse::try_from(31u8).unwrap()
        );
    }

    #[test]
    fn test_range() {
        let bible = Bible::new();

        let range = bible.range(BibleEnum::Exodus..BibleEnum::Numbers);

        let result: [&str; 3] = ["Exodus", "Leviticus", "Numbers"];

        for (i, (_id, book)) in range.enumerate() {
            assert_eq!(book.name.to_string(), result[i])
        }
    }
}
