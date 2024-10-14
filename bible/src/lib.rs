use std::{collections::BTreeMap, ops::{Index, IndexMut}};

#[derive(Debug)]
pub struct Bible {
    books: BTreeMap<BookName, Book>;
}

impl Bible {
    pub new() -> Bible {
        Bible {
            books: BTreeMap::new(),
        }
    }
}

impl Index<BookName> for Bible {
    type Output = Book;

    fn index(&self, index: BookName) -> &Self::Output {
        self.books.get(&index).unwrap()
    }
}

impl Index<&BookName> for Bible {
    type Output = Book;

    fn index(&self, index: &BookName) -> &Self::Output {
        &self[*index]
    }
}

impl IndexMut<BookName> for Bible {
    fn index_mut(&mut self, index: BookName) -> &mut Self::Output {
        self.books.get_mut(&index).unwrap()
    }
}

impl IndexMut<&BookName> for Bible {
    fn index_mut(&mut self, index: &BookName) -> &mut Self::Output {
        &mut self[*index]
    }
}

impl<'a> IntoIterator for &'a Bible {
    type Item = &'a Book;
    type IntoIter = std::collections::btree_map::Values<'a, BookName, Book>;

    fn into_iter(self) -> Self::IntoIter {
        self.books.values()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_work() {
        let mut store: Bible = Bible::new();
    }
}
