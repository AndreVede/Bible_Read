#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Verse(u8);

#[derive(Debug, thiserror::Error)]
pub enum VerseError {
    #[error("The verse cannot be zero")]
    Zero,
    #[error("The verse cannot be greater than 176")]
    Max,
}

impl TryFrom<u8> for Verse {
    type Error = VerseError;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        validate(&value)?;
        Ok(Self(value))
    }
}

impl TryFrom<&u8> for Verse {
    type Error = VerseError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        validate(value)?;
        Ok(Self(*value))
    }
}

impl From<Verse> for u8 {
    fn from(verse: Verse) -> Self {
        verse.0
    }
}

impl From<&Verse> for u8 {
    fn from(verse: &Verse) -> Self {
        u8::from(*verse)
    }
}

fn validate(number: &u8) -> Result<(), VerseError> {
    if *number == 0 {
        Err(VerseError::Zero)
    } else if *number > 176 {
        Err(VerseError::Max)
    } else {
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Verse;

    #[test]
    fn test_try_from_u8() {
        let verse = Verse::try_from(176u8).unwrap();

        assert_eq!(verse.0, 176u8);
    }

    #[test]
    fn test_try_from_ref_u8() {
        let verse = Verse::try_from(&176u8).unwrap();

        assert_eq!(verse.0, 176u8);
    }

    #[test]
    fn test_try_from_zero() {
        let verse = Verse::try_from(0u8).unwrap_err();

        assert_eq!(verse.to_string(), "The verse cannot be zero");
    }

    #[test]
    fn test_try_from_max() {
        let max_verse = Verse::try_from(&177u8).unwrap_err();

        assert_eq!(
            max_verse.to_string(),
            "The verse cannot be greater than 176"
        );
    }

    #[test]
    fn test_from_verse_u8() {
        let verse: Verse = Verse::try_from(3u8).unwrap();
        assert_eq!(3u8, verse.into());
    }

    #[test]
    fn test_from_verse_ref_u8() {
        let verse: Verse = Verse::try_from(3u8).unwrap();

        assert_eq!(3u8, u8::from(&verse));
    }
}
