pub mod book_components;

use book_components::{chapter_store::ChapterStore, name::BookName};
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct Book {
    pub name: BookName,
    pub chapters: ChapterStore,
}
