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
        path: Arc<String>,
        response_channel: SyncSender<Result<(), SaveServerError>>,
    },
    GetReadingFromFile {
        path: Arc<String>,
        response_channel: SyncSender<Result<Reading, SaveServerError>>,
    },
    GetCurrentReading {
        response_channel: SyncSender<Arc<Mutex<Option<Reading>>>>,
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
}

#[derive(Clone)]
pub struct ReadingSaveClient {
    path: Arc<String>,
    sender: SyncSender<Command>,
}

impl ReadingSaveClient {
    pub fn get_reading(&self) -> Result<Reading, SaveServerError> {
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

    pub fn save_reading(&self) -> Result<(), SaveServerError> {
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
}

pub fn launch_reading(capacity: usize, path: String) -> ReadingSaveClient {
    let (sender, receiver) = sync_channel(capacity);
    spawn(move || server_reading(receiver));
    ReadingSaveClient {
        sender,
        path: Arc::new(path),
    }
}

fn server_reading(receiver: Receiver<Command>) {
    // The current Reading Value
    let reading: Arc<Mutex<Option<Reading>>> = Arc::new(Mutex::new(None));

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
                            if let Ok(ref mut reading_value) = reading.try_lock() {
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
                let reading_lock = reading.lock().unwrap();

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
                        // Return Not Possible
                        let _ = response_channel.send(Err(SaveServerError::OverloadedError));
                    }
                }
            }
            Ok(Command::GetCurrentReading { response_channel }) => {
                let _ = response_channel.send(reading.clone());
            }
            Err(_) => {
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_that_work() {
        todo!();
    }
}
