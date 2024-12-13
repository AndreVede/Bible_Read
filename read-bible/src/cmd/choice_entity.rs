use clap::ValueEnum;
use serde::Serialize;

#[derive(ValueEnum, Clone, Default, Debug, Serialize)]
#[serde(rename_all = "kebab-case")]
pub enum ChoiceEntity {
    /// The Book entity
    Book,
    /// The Chapter entity
    Chapter,
    /// The Verse entity
    #[default]
    Verse,
}
