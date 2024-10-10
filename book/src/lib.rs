mod book_components;

use book_components::name::BookName;

#[derive(Debug, PartialEq, Clone)]
pub struct Book {
    pub name: BookName,
}
