#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct MaxVerse(u8);

#[derive(Debug, thiserror::Error)]
pub enum MaxVerseError {
    #[error("The max verse cannot be zero")]
    Zero,
    #[error("The max verse cannot be greater than 176")]
    Max,
}

impl TryFrom<u8> for MaxVerse {
    type Error = MaxVerseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        validate(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&u8> for MaxVerse {
    type Error = MaxVerseError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        validate(value)?;
        Ok(Self(*value))
    }
}

impl From<MaxVerse> for u8 {
    fn from(max_verse: MaxVerse) -> Self {
        max_verse.0
    }
}

impl From<&MaxVerse> for u8 {
    fn from(max_verse: &MaxVerse) -> Self {
        u8::from(*max_verse)
    }
}

fn validate(number: &u8) -> Result<(), MaxVerseError> {
    if *number == 0 {
        Err(MaxVerseError::Zero)
    } else if *number > 176 {
        Err(MaxVerseError::Max)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::MaxVerse;

    #[test]
    fn test_try_from_u8() {
        let max_verse = MaxVerse::try_from(176u8).unwrap();

        assert_eq!(max_verse.0, 176u8);
    }

    #[test]
    fn test_try_from_ref_u8() {
        let max_verse = MaxVerse::try_from(&176u8).unwrap();

        assert_eq!(max_verse.0, 176u8);
    }

    #[test]
    fn test_try_from_zero() {
        let max_verse = MaxVerse::try_from(0u8).unwrap_err();

        assert_eq!(max_verse.to_string(), "The max verse cannot be zero");
    }

    #[test]
    fn test_try_from_max() {
        let max_verse = MaxVerse::try_from(&177u8).unwrap_err();

        assert_eq!(
            max_verse.to_string(),
            "The max verse cannot be greater than 176"
        );
    }

    #[test]
    fn test_from_max_verse_u8() {
        let max_verse: MaxVerse = MaxVerse::try_from(3u8).unwrap();
        assert_eq!(3u8, max_verse.into());
    }

    #[test]
    fn test_from_max_verse_ref_u8() {
        let max_verse: MaxVerse = MaxVerse::try_from(3u8).unwrap();

        assert_eq!(3u8, u8::from(&max_verse));
    }
}
