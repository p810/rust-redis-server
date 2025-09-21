use std::collections::{VecDeque};
use std::io::{Error, ErrorKind, Read, Write};
use std::{thread, time::Duration};
use std::net::{TcpListener, TcpStream};
use std::sync::mpsc::{channel, Receiver, TryRecvError};

use crate::resp::commands::get_command_from_input;
use crate::worker::{spawn_worker, WorkerMessage, WorkerResponse};

mod resp;
mod store;
mod worker;

struct Client {
    stream: TcpStream,
    read_buffer: Vec<u8>,
    pending: VecDeque<Receiver<WorkerResponse>>,
}

fn main() -> Result<(), Error> {
    println!("Server started, now listening for connections...");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    listener.set_nonblocking(true)?;

    let mut conns: Vec<Client> = Vec::new();

    let worker_tx = spawn_worker();

    loop {
        match listener.accept() {
            Ok((stream, _addr)) => {
                stream.set_nonblocking(true)?;

                println!("Accepting new connection");

                conns.push(Client {
                    stream,
                    read_buffer: vec![0; 1024],
                    pending: VecDeque::new(),
                });
            }

            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                // no new connection this iteration, do nothing
            }

            Err(e) => return Err(e)
        }

        let mut closed_connections = Vec::new();

        for (i, client) in conns.iter_mut().enumerate() {
            let mut stream = &client.stream;
            let mut buf = &mut client.read_buffer;

            match stream.read(&mut buf) {
                Ok(0) => closed_connections.push(i),

                Ok(n) => {
                    println!("Read {n} bytes");

                    match get_command_from_input(&buf[..n]) {
                        Ok(command) => {
                            let (response_tx, response_rx) = channel::<WorkerResponse>();

                            let message = WorkerMessage {
                                op: command,
                                reply: response_tx,
                            };

                            client.pending.push_back(response_rx);

                            if let Err(e) = worker_tx.send(message) {
                                eprintln!("Unable to send message to worker thread");
                                dbg!(e);
                            };
                        }
                        Err(_) => {
                            // to do: better error handling
                            stream.write_all("-ERR an error occurred\r\n".as_bytes())?;
                        }
                    }
                }

                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    // do nothing
                }

                Err(_) => {
                    closed_connections.push(i);
                }
            }

            while let Some(front) = client.pending.front() {
                match front.try_recv() {
                    Ok(response) => {
                        client.pending.pop_front();

                        if let Ok(payload) = response {
                            if let Some(data) = payload {
                                client.stream.write_all(&data)?;

                                let outgoing = data.len();
                                println!("Wrote {outgoing} bytes");
                            }
                        }
                    }
                    Err(e) => {
                        if e != TryRecvError::Empty {
                            eprintln!("Error while draining pending responses");
                            dbg!(e);
                        }
                    }
                }
            };
        }

        for i in closed_connections {
            conns.swap_remove(i);
        }

        // 5ms backoff to prevent busy waiting
        thread::sleep(Duration::from_millis(5));
    }
}
