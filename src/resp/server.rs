use crate::resp::commands::{
    RespCommand,
    RespCommandError,
    get_command_from_input,
};

use std::collections::HashMap;

pub fn respond_to_request(packet: &[u8], store: &mut HashMap<String, Vec<u8>>) -> Result<Vec<u8>, RespCommandError> {
    match get_command_from_input(packet) {
        Ok(command) => {
            match command {
                RespCommand::Ping => Ok("+PONG\r\n".into()),
                RespCommand::Echo(e) => {
                    let response = format!("${}\r\n{}", e.value.len(), e.value);
                    
                    Ok(response.as_bytes().to_vec())
                }
                RespCommand::Set(s) => {
                    store.insert(s.key, s.value.as_bytes().to_vec());
                    
                    Ok("+OK\r\n".into())
                }
                RespCommand::Get(g) => {
                    let Some(value) = store.get(&g.key) else {
                        return Ok("$-1\r\n".into());
                    };

                    let mut response = vec![b'$'];
                    let separator = &[b'\r', b'\n'];

                    // Add the length and then start a new line
                    response.extend(value.len().to_string().as_bytes());
                    response.extend(separator);

                    // Add the key's stored contents and a final new line
                    response.extend(value);
                    response.extend(separator);

                    Ok(response)
                }
            }
        }
        _ => Err(RespCommandError::InvalidArgument),
    }
}