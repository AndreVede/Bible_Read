#[derive(Debug, PartialEq, Clone)]
pub struct BookName(String);

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum BookNameError {
    #[error("The book name cannot be empty")]
    Empty,
    #[error("The book name cannot be longer than 50 bytes")]
    TooLong
}

impl std::fmt::Display for BookName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl TryFrom<String> for BookName {
    type Error = BookNameError;

    fn try_from(value: String)-> Result<Self, Self::Error> {
        validate(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&str> for BookName {
    type Error = BookNameError;

    fn try_from(value: &str)-> Result<Self, Self::Error> {
        validate(value)?;
        Ok(Self(value.to_string()))
    }
}

fn validate(name: &str) -> Result<(), BookNameError> {
    if name.is_empty() {
        Err(BookNameError::Empty)
    } else if name.len() > 50 {
        Err(BookNameError::TooLong)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::convert::TryFrom;

    #[test]
    fn test_try_from_string() {
        let book_name: BookName = BookName::try_from("book name".to_string()).unwrap();
        assert_eq!(book_name.0, "book name");
    }

    #[test]
    fn test_try_from_str() {
        let book_name: BookName = BookName::try_from("book name").unwrap();
        assert_eq!(book_name.to_string(), "book name");
    }

    #[test]
    fn test_try_from_empty() {
        let book_name: BookNameError = BookName::try_from("").unwrap_err();
        assert_eq!(book_name.to_string(), "The book name cannot be empty");
    }

    #[test]
    fn test_try_from_too_long() {
        let book_name: BookNameError = BookName::try_from("Lorem ipsum dolor sit amet, consectetur vestibulum.").unwrap_err();
        assert_eq!(book_name.to_string(), "The book name cannot be longer than 50 bytes");
    }
}