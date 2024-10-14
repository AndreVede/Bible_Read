#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct ChapterNumber(u8);

#[derive(Debug, thiserror::Error)]
pub enum ChapterNumberError {
    #[error("The chapter number cannot be zero")]
    Zero,
    #[error("The chapter number cannot be greater than 150")]
    Max
}

impl TryFrom<u8> for ChapterNumber {
    type Error = ChapterNumberError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        validate(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&u8> for ChapterNumber {
    type Error = ChapterNumberError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        validate(value)?;
        Ok(Self(*value))
    }
}

impl From<ChapterNumber> for u8 {
    fn from(chapter_number: ChapterNumber) -> Self {
        chapter_number.0
    }
}

impl From<&ChapterNumber> for u8 {
    fn from(chapter_number: &ChapterNumber) -> Self {
        u8::from(*chapter_number)
    }
}

fn validate(number: &u8) -> Result<(), ChapterNumberError> {
    if *number == 0 {
        Err(ChapterNumberError::Zero)
    } else if *number >= 150 {
        Err(ChapterNumberError::Max)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_try_from_u8() {
        let chapter_number = ChapterNumber::try_from(37u8).unwrap();
        assert_eq!(chapter_number.0, 37u8);
    }

    #[test]
    fn test_try_from_ref_u8() {
        let chapter_number = ChapterNumber::try_from(&37u8).unwrap();
        assert_eq!(chapter_number.0, 37u8);
    }

    #[test]
    fn test_try_from_zero() {
        let chapter_number = ChapterNumber::try_from(&0u8).unwrap_err();
        assert_eq!(chapter_number.to_string(), "The chapter number cannot be zero");
    }

    #[test]
    fn test_try_from_max() {
        let chapter_number = ChapterNumber::try_from(&151u8).unwrap_err();
        assert_eq!(chapter_number.to_string(), "The chapter number cannot be greater than 150");
    }

    #[test]
    fn test_from_chapter_number_u8() {
        let chapter_number: ChapterNumber = ChapterNumber::try_from(3u8).unwrap();
        assert_eq!(3u8, chapter_number.into());
    }

    #[test]
    fn test_from_ref_chapter_number_u8() {
        let chapter_number: ChapterNumber = ChapterNumber::try_from(3u8).unwrap();
        assert_eq!(3u8, u8::from(&chapter_number));
    }
}
