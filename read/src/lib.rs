mod file_operations;
pub mod reading;

use std::{
    sync::{
        mpsc::{sync_channel, Receiver, SyncSender},
        Arc, Mutex,
    },
    thread::spawn,
};

use file_operations::{get_reading_in_file, save_reading_in_file};
use reading::Reading;

#[derive(Debug, Clone)]
enum Command {
    SaveReadingInFile {
        path: Arc<std::path::PathBuf>,
        response_channel: SyncSender<Result<(), SaveServerError>>,
    },
    GetReadingFromFile {
        path: Arc<std::path::PathBuf>,
        response_channel: SyncSender<Result<Reading, SaveServerError>>,
    },
    GetCurrentReading {
        response_channel: SyncSender<Arc<Mutex<Option<Reading>>>>,
    },
    SetCurrentReading {
        reading: Reading,
        response_channel: SyncSender<()>,
    },
}

#[derive(Debug, thiserror::Error)]
pub enum SaveServerError {
    #[error("The save server is overloaded")]
    OverloadedError,
    #[error("Saving in file failed")]
    FailedToSave,
    #[error("Getting the save from file failed")]
    FailedToGetSave,
    #[error("No data to save, reading is none")]
    NoDataToSave,
}

#[derive(Clone)]
pub struct ReadingSaveClient {
    path: Arc<std::path::PathBuf>,
    sender: SyncSender<Command>,
}

impl ReadingSaveClient {
    pub fn get_reading_from_file(&self) -> Result<Reading, SaveServerError> {
        let (response_sender, response_receiver) = sync_channel(1);
        self.sender
            .try_send(Command::GetReadingFromFile {
                path: self.path.clone(),
                response_channel: response_sender,
            })
            .map_err(|_| SaveServerError::OverloadedError)?;

        let reading = response_receiver.recv().unwrap()?;

        Ok(reading)
    }

    pub fn save_reading_in_file(&self) -> Result<(), SaveServerError> {
        let (response_sender, response_receiver) = sync_channel(1);
        self.sender
            .try_send(Command::SaveReadingInFile {
                path: self.path.clone(),
                response_channel: response_sender,
            })
            .map_err(|_| SaveServerError::OverloadedError)?;

        response_receiver.recv().unwrap()?;

        Ok(())
    }

    pub fn get_current_reading(&self) -> Result<Arc<Mutex<Option<Reading>>>, SaveServerError> {
        let (response_sender, response_receiver) = sync_channel(1);
        self.sender
            .try_send(Command::GetCurrentReading {
                response_channel: response_sender,
            })
            .map_err(|_| SaveServerError::OverloadedError)?;

        Ok(response_receiver.recv().unwrap())
    }

    pub fn set_current_reading(&self, reading: Reading) -> Result<(), SaveServerError> {
        let (response_sender, response_receiver) = sync_channel(1);
        self.sender
            .try_send(Command::SetCurrentReading {
                reading,
                response_channel: response_sender,
            })
            .map_err(|_| SaveServerError::OverloadedError)?;

        response_receiver.recv().unwrap();
        Ok(())
    }
}

pub fn launch_reading(capacity: usize, path: std::path::PathBuf) -> ReadingSaveClient {
    let (sender, receiver) = sync_channel(capacity);
    spawn(move || server_reading(receiver));
    ReadingSaveClient {
        sender,
        path: Arc::new(path),
    }
}

fn server_reading(receiver: Receiver<Command>) {
    // The current Reading Value
    let current_reading: Arc<Mutex<Option<Reading>>> = Arc::new(Mutex::new(None));

    loop {
        match receiver.recv() {
            Ok(Command::GetReadingFromFile {
                path,
                response_channel,
            }) => {
                match get_reading_in_file(path) {
                    Ok(function_result) => {
                        if let Some(reading_result) = function_result {
                            // Save the reading value in current value
                            if let Ok(ref mut reading_value) = current_reading.try_lock() {
                                **reading_value = Some(reading_result.clone());
                            }

                            let _ = response_channel.send(Ok(reading_result));
                        }

                        let _ = response_channel.send(Err(SaveServerError::FailedToGetSave));
                    }
                    Err(_) => response_channel
                        .send(Err(SaveServerError::FailedToGetSave))
                        .unwrap(),
                }
            }
            Ok(Command::SaveReadingInFile {
                path,
                response_channel,
            }) => {
                let reading_lock = current_reading.lock().unwrap();

                match *reading_lock {
                    Some(ref reading_value) => {
                        if save_reading_in_file(path, reading_value).is_ok() {
                            let _ = response_channel.send(Ok(()));
                        } else {
                            // Return failed
                            let _ = response_channel.send(Err(SaveServerError::FailedToSave));
                        }
                    }
                    None => {
                        // No data
                        let _ = response_channel.send(Err(SaveServerError::NoDataToSave));
                    }
                }
            }
            Ok(Command::GetCurrentReading { response_channel }) => {
                let _ = response_channel.send(current_reading.clone());
            }
            Ok(Command::SetCurrentReading {
                reading,
                response_channel,
            }) => {
                let mut reading_lock = current_reading.lock().unwrap();

                *reading_lock = Some(reading);

                let _ = response_channel.send(());
            }
            Err(_) => {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use bible::bible_enum::BibleEnum;
    use book::book_components::{chapter_number::ChapterNumber, verse::Verse};

    use super::*;

    #[test]
    fn test_that_work() {
        let client = launch_reading(1, "test1.ron".into());

        let reading = Reading::new(
            BibleEnum::Genesis,
            ChapterNumber::try_from(1u8).unwrap(),
            Verse::try_from(1u8).unwrap(),
        )
        .unwrap();

        let _ = client.set_current_reading(reading.clone());

        let current_reading = client.get_current_reading().unwrap();

        let current_reading_lock = current_reading.lock().unwrap();

        assert_eq!(Some(reading), *current_reading_lock);
    }
}
