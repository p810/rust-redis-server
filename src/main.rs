use std::collections::HashMap;
use std::io::{Error, ErrorKind, Read, Write};
use std::{thread, time::Duration};
use std::net::{TcpListener, TcpStream};
use crate::resp::server::respond_to_request;

mod resp;

fn main() -> Result<(), Error> {
    println!("Server started, now listening for connections...");

    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    listener.set_nonblocking(true)?;

    let mut clients: HashMap<u64, TcpStream> = HashMap::new();
    let mut next_id: u64 = 1;

    let mut buf = [0u8; 1024];

    let mut store: HashMap<String, Vec<u8>> = HashMap::new();

    loop {
        match listener.accept() {
            Ok((stream, _addr)) => {
                stream.set_nonblocking(true)?;

                let id = next_id;
                next_id += 1;

                clients.insert(id, stream);
            }

            Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                // no new connection this iteration, do nothing
            }

            Err(e) => return Err(e)
        }

        let mut closed_connections = Vec::new();

        for (id, stream) in clients.iter_mut() {
            match stream.read(&mut buf) {
                Ok(0) => closed_connections.push(*id),

                Ok(n) => {
                    match respond_to_request(&buf[..n], &mut store) {
                        Ok(data) => {
                            stream.write_all(&data)?;
                        }
                        Err(e) => {
                            dbg!(e);
                        }
                    }
                }

                Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                    // do nothing
                }

                Err(_) => closed_connections.push(*id)
            }
        }

        for id in closed_connections {
            clients.remove(&id);
        }

        // 5ms backoff to prevent busy waiting
        thread::sleep(Duration::from_millis(5));
    }
}
