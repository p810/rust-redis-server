use std::sync::mpsc::{channel, Sender, RecvTimeoutError};
use std::thread;
use std::time::Duration;

use crate::store::Database;
use crate::resp::commands::{RespCommand, RespCommandError};

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
                            let response = format!("${}\r\n{}", e.value.len(), e.value);
                            
                            Some(response.as_bytes().to_vec())
                        }
                        RespCommand::Set(s) => {
                            db.set(s.key.as_str(), &s.value, s.ttl);

                            Some("+OK\r\n".into())
                        }
                        RespCommand::Get(g) => {
                            if let Some(entry) = db.get(g.key.as_str()) {
                                let mut response = vec![b'$'];
                                let separator = &[b'\r', b'\n'];

                                // Add the length and then start a new line
                                response.extend(entry.value.len().to_string().as_bytes());
                                response.extend(separator);

                                // Add the key's stored contents and a final new line
                                response.extend(entry.value.clone());
                                response.extend(separator);

                                Some(response)
                            } else {
                                Some("$-1\r\n".into())
                            }
                        }
                    };

                    match reply.send(Ok(response)) {
                        Err(e) => {
                            eprintln!("Unable to send response back to main thread");
                            dbg!(e);
                        }
                        _ => {},
                    };
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