use bible::bible_enum::BibleEnum;
use clap::Args;
use read::{
    launch_reading,
    reading::{Reading, ReadingError},
};

use super::choice_entity::ChoiceEntity;

pub enum Direction {
    Next,
    Previous,
}

#[derive(Debug, Args)]
#[command(version, about, long_about = None)]
pub struct NextPreviousArgs {
    #[arg(short, long, default_value_t, value_enum)]
    entity: ChoiceEntity,
    #[arg(short, long, default_value = "1")]
    count: u8,
}

impl NextPreviousArgs {
    pub fn next(&self, reading: &mut Reading) -> Result<(), ReadingError> {
        match self.entity {
            ChoiceEntity::Book => reading.next_book(&self.count),
            ChoiceEntity::Chapter => reading.next_chapter(&self.count),
            ChoiceEntity::Verse => reading.next_verse(&self.count),
        }
    }

    pub fn previous(&self, reading: &mut Reading) -> Result<(), ReadingError> {
        match self.entity {
            ChoiceEntity::Book => reading.previous_book(&self.count),
            ChoiceEntity::Chapter => reading.previous_chapter(&self.count),
            ChoiceEntity::Verse => reading.previous_verse(&self.count),
        }
    }
}

pub fn main(
    path: std::path::PathBuf,
    args: &NextPreviousArgs,
    direction: Direction,
) -> anyhow::Result<()> {
    let client = launch_reading(1, path);

    let mut new_reading: Reading = Reading::new(
        BibleEnum::Genesis,
        1u8.try_into().unwrap(),
        1u8.try_into().unwrap(),
    )
    .unwrap();

    if let Ok(existant_reading) = client.get_reading_from_file() {
        new_reading = existant_reading;
        match direction {
            Direction::Next => args.next(&mut new_reading).unwrap(),
            Direction::Previous => args.previous(&mut new_reading).unwrap(),
        };
    }

    client
        .set_current_reading(new_reading)
        .expect("There was a problem when we set the new reading");
    client.save_reading_in_file()?;

    Ok(())
}
