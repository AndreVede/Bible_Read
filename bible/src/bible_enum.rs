use serde::{Deserialize, Serialize};
use strum::EnumIter;

macro_rules! make_bible_enum {
    ($error: ident ($message: literal), $enum_name: ident {$($book_ident: ident),* $(,)*}) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, EnumIter, Serialize, Deserialize)]
        #[serde(rename_all = "kebab-case")]
        pub enum $enum_name {
            $($book_ident),*
        }

        #[derive(Debug, thiserror::Error)]
        #[error($message)]
        pub struct $error;

        impl std::str::FromStr for $enum_name {
            type Err = $error;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                match s {
                    $(stringify!($book_ident) => Ok($enum_name::$book_ident),)*
                    _ => Err($error),
                }
            }
        }
    };
}

make_bible_enum!(
    BibleEnumError("This is not a Bible book."),
    BibleEnum {
        Genesis,
        Exodus,
        Leviticus,
        Numbers,
        Deuteronomy,
        Joshua,
        Judges,
        Ruth,
        FirstSamuel,
        SecondSamuel,
        FirstKings,
        SecondKings,
        FirstChronicles,
        SecondChronicles,
        Ezra,
        Nehemiah,
        Esther,
        Job,
        Psalms,
        Proverbs,
        Ecclesiastes,
        SongOfSolomon,
        Isaiah,
        Jeremiah,
        Lamentations,
        Ezekiel,
        Daniel,
        Hosea,
        Joel,
        Amos,
        Obadiah,
        Jonah,
        Micah,
        Nahum,
        Habakkuk,
        Zephaniah,
        Haggai,
        Zechariah,
        Malachi,
        Matthew,
        Mark,
        Luke,
        John,
        Acts,
        Romans,
        FirstCorinthians,
        SecondCorinthians,
        Galatians,
        Ephesians,
        Philippians,
        Colossians,
        FirstThessalonians,
        SecondThessalonians,
        FirstTimothy,
        SecondTimothy,
        Titus,
        Philemon,
        Hebrews,
        James,
        FirstPeter,
        SecondPeter,
        FirstJohn,
        SecondJohn,
        ThirdJohn,
        Jude,
        Revelation,
    }
);
