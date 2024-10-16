pub mod book_components;

use book_components::{chapter_store::ChapterStore, name::BookName};

#[derive(Debug, PartialEq, Clone)]
pub struct Book {
    pub name: BookName,
    pub chapters: ChapterStore,
}
