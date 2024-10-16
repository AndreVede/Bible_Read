use book::Book;
use init_bible::init_books;
use std::{
    collections::BTreeMap,
    ops::{Index, IndexMut},
};

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

        for (index, book) in data.iter().enumerate() {
            match index {
                0usize => books.insert(BibleEnum::Genesis, book.clone()),
                1usize => books.insert(BibleEnum::Exodus, book.clone()),
                2usize => books.insert(BibleEnum::Leviticus, book.clone()),
                3usize => books.insert(BibleEnum::Numbers, book.clone()),
                4usize => books.insert(BibleEnum::Deuteronomy, book.clone()),
                5usize => books.insert(BibleEnum::Joshua, book.clone()),
                6usize => books.insert(BibleEnum::Judges, book.clone()),
                7usize => books.insert(BibleEnum::Ruth, book.clone()),
                8usize => books.insert(BibleEnum::FirstSamuel, book.clone()),
                9usize => books.insert(BibleEnum::SecondSamuel, book.clone()),
                10usize => books.insert(BibleEnum::FirstKings, book.clone()),
                11usize => books.insert(BibleEnum::SecondKings, book.clone()),
                12usize => books.insert(BibleEnum::FirstChronicles, book.clone()),
                13usize => books.insert(BibleEnum::SecondChronicles, book.clone()),
                14usize => books.insert(BibleEnum::Ezra, book.clone()),
                15usize => books.insert(BibleEnum::Nehemiah, book.clone()),
                16usize => books.insert(BibleEnum::Esther, book.clone()),
                17usize => books.insert(BibleEnum::Job, book.clone()),
                18usize => books.insert(BibleEnum::Psalms, book.clone()),
                19usize => books.insert(BibleEnum::Proverbs, book.clone()),
                20usize => books.insert(BibleEnum::Ecclesiastes, book.clone()),
                21usize => books.insert(BibleEnum::SongOfSolomon, book.clone()),
                22usize => books.insert(BibleEnum::Isaiah, book.clone()),
                23usize => books.insert(BibleEnum::Jeremiah, book.clone()),
                24usize => books.insert(BibleEnum::Lamentations, book.clone()),
                25usize => books.insert(BibleEnum::Ezekiel, book.clone()),
                26usize => books.insert(BibleEnum::Daniel, book.clone()),
                27usize => books.insert(BibleEnum::Hosea, book.clone()),
                28usize => books.insert(BibleEnum::Joel, book.clone()),
                29usize => books.insert(BibleEnum::Amos, book.clone()),
                30usize => books.insert(BibleEnum::Obadiah, book.clone()),
                31usize => books.insert(BibleEnum::Jonah, book.clone()),
                32usize => books.insert(BibleEnum::Micah, book.clone()),
                33usize => books.insert(BibleEnum::Nahum, book.clone()),
                34usize => books.insert(BibleEnum::Habakkuk, book.clone()),
                35usize => books.insert(BibleEnum::Zephaniah, book.clone()),
                36usize => books.insert(BibleEnum::Haggai, book.clone()),
                37usize => books.insert(BibleEnum::Zechariah, book.clone()),
                38usize => books.insert(BibleEnum::Malachi, book.clone()),
                39usize => books.insert(BibleEnum::Matthew, book.clone()),
                40usize => books.insert(BibleEnum::Mark, book.clone()),
                41usize => books.insert(BibleEnum::Luke, book.clone()),
                42usize => books.insert(BibleEnum::John, book.clone()),
                43usize => books.insert(BibleEnum::Acts, book.clone()),
                44usize => books.insert(BibleEnum::Romans, book.clone()),
                45usize => books.insert(BibleEnum::FirstCorinthians, book.clone()),
                46usize => books.insert(BibleEnum::SecondCorinthians, book.clone()),
                47usize => books.insert(BibleEnum::Galatians, book.clone()),
                48usize => books.insert(BibleEnum::Ephesians, book.clone()),
                49usize => books.insert(BibleEnum::Philippians, book.clone()),
                50usize => books.insert(BibleEnum::Colossians, book.clone()),
                51usize => books.insert(BibleEnum::FirstThessalonians, book.clone()),
                52usize => books.insert(BibleEnum::SecondThessalonians, book.clone()),
                53usize => books.insert(BibleEnum::FirstTimothy, book.clone()),
                54usize => books.insert(BibleEnum::SecondTimothy, book.clone()),
                55usize => books.insert(BibleEnum::Titus, book.clone()),
                56usize => books.insert(BibleEnum::Philemon, book.clone()),
                57usize => books.insert(BibleEnum::Hebrews, book.clone()),
                58usize => books.insert(BibleEnum::James, book.clone()),
                59usize => books.insert(BibleEnum::FirstPeter, book.clone()),
                60usize => books.insert(BibleEnum::SecondPeter, book.clone()),
                61usize => books.insert(BibleEnum::FirstJohn, book.clone()),
                62usize => books.insert(BibleEnum::SecondJohn, book.clone()),
                63usize => books.insert(BibleEnum::ThirdJohn, book.clone()),
                64usize => books.insert(BibleEnum::Jude, book.clone()),
                65usize => books.insert(BibleEnum::Revelation, book.clone()),
                _ => panic!("The Bible has only 66 books... init failed"),
            };
        }

        Bible { books }
    }
}

impl Bible {
    pub fn new() -> Bible {
        Default::default()
    }
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
}
