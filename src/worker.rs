use std::sync::mpsc::{channel, Sender, RecvTimeoutError};
use std::thread;
use std::time::Duration;

use crate::resp::parser::RespSerialize;
use crate::resp::types::RespBulkString;
use crate::resp::commands::{RespCommand, RespCommandError};
use crate::resp::{RESP_EMPTY_STRING, RESP_OK};
use crate::store::Database;

pub struct WorkerMessage {
    pub op: RespCommand,
    pub reply: Sender<WorkerResponse>,
}

pub type WorkerResponse = Result<Option<Vec<u8>>, RespCommandError>;

pub fn spawn_worker() -> Sender<WorkerMessage> {
    let (worker_tx, worker_rx) = channel::<WorkerMessage>();

    thread::spawn(move || {
        let mut db = Database::new();

        loop {
            let timeout = db.time_until_next_expiration();

            let cmd = match timeout {
                Some(dur) if dur > Duration::from_millis(0) =>
                    worker_rx.recv_timeout(dur),
                Some(_) =>
                    Err(RecvTimeoutError::Timeout),
                None =>
                    worker_rx.recv().map_err(| _ | RecvTimeoutError::Disconnected),
            };

            match cmd {
                Ok(WorkerMessage { op, reply }) => {
                    let response = match op {
                        RespCommand::Ping => {
                            Some("+PONG\r\n".into())
                        },
                        RespCommand::Echo(e) => {
                            let response = RespBulkString::new(e.value.as_bytes());
                            
                            Some(response.to_bytes())
                        }
                        RespCommand::Set(s) => {
                            db.set(s.key.as_str(), &s.value, s.ttl);

                            Some(RESP_OK.to_vec())
                        }
                        RespCommand::Get(g) => {
                            if let Some(entry) = db.get(g.key.as_str()) {
                                let response = RespBulkString::new(&entry.value);

                                Some(response.to_bytes())
                            } else {
                                Some(RESP_EMPTY_STRING.to_vec())
                            }
                        }
                    };

                    if let Err(e) = reply.send(Ok(response)) {
                        eprintln!("Unable to send response back to main thread");
                        dbg!(e);
                    }
                }
                Err(RecvTimeoutError::Timeout) => {
                    // to do: make the budget configurable
                    db.delete_expired_keys(1_000);
                }
                Err(_) => break
            };

            thread::sleep(Duration::from_millis(5));
        }

        println!("Closing worker thread...");
    });

    worker_tx
}