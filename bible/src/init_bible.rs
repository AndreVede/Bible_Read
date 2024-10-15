use book::{
    book_components::{chapter::Chapter, chapter_store::ChapterStore},
    Book,
};
use serde::{Deserialize, Serialize};

use crate::constants::ENGLISH_BOOK_NAME_LIST;

macro_rules! make_bible_json_struct {
    ($number_of_keys: expr, $struct_name: ident {$($book_id: ident),* $(,)*}) => {
        #[derive(Debug, Deserialize, Serialize, Clone)]
        struct $struct_name {
            $($book_id: Vec<String>),*
        }

        impl IntoIterator for $struct_name {
            type Item = Vec<String>;
            type IntoIter = std::array::IntoIter<Self::Item, $number_of_keys>;

            fn into_iter(self: $struct_name) -> Self::IntoIter {
                [$(self.$book_id),*].into_iter()
            }
        }
    }
}

make_bible_json_struct!(
    66usize,
    BibleJson {
        gen,
        exo,
        lev,
        num,
        deut,
        jos,
        judg,
        ruth,
        ones,
        twos,
        oner,
        twor,
        onech,
        twoch,
        esd,
        neh,
        esther,
        job,
        psa,
        pro,
        ecc,
        song,
        isa,
        jerem,
        lam,
        eze,
        dan,
        os,
        joel,
        amos,
        abd,
        jona,
        mic,
        nah,
        hab,
        soph,
        ag,
        zach,
        malac,
        mat,
        mark,
        luke,
        john,
        acts,
        rom,
        oneco,
        twoco,
        gal,
        eph,
        philip,
        col,
        onethe,
        twothe,
        oneti,
        twoti,
        titus,
        philemon,
        heb,
        jac,
        onep,
        twop,
        onej,
        twoj,
        threej,
        jude,
        rev
    }
);

fn get_bible_json() -> BibleJson {
    let bible_str = include_str!("bible.json");
    serde_json::from_str(bible_str).unwrap()
}

pub fn init_books() -> Vec<Book> {
    let bible_json: BibleJson = get_bible_json();
    let mut vec: Vec<Book> = Vec::new();
    for (index_book, book) in bible_json.into_iter().enumerate() {
        let mut chapter_store = ChapterStore::new();

        for (index, verse_str) in book.iter().enumerate() {
            let verse: u8 = verse_str.parse().unwrap();
            let chapter_number: u8 = index as u8 + 1u8;

            chapter_store.add_chapter(Chapter::new(
                chapter_number.try_into().unwrap(),
                verse.try_into().unwrap(),
            ));
        }

        vec.push(Book {
            name: ENGLISH_BOOK_NAME_LIST[index_book].try_into().unwrap(),
            chapters: chapter_store,
        });
    }
    vec
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_work() {
        let mut data_iter = get_bible_json().into_iter();

        let gen_getted_by_iter = data_iter.next().unwrap();

        assert_eq!(gen_getted_by_iter.first().unwrap(), "31");
    }
}
