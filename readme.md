# rust-redis-server

This is a toy redis server that I've been working on in my spare time, mostly as a way to get more familiar with Rust.

I first started working on this back in 2023, but dropped it early on since I didn't have enough free time to fully get into it. I restarted the project last week, and have been working on it while I've been reading through [The Rust Programming Language](https://doc.rust-lang.org/book/title-page.html).

As of right now this is still essentially a rough draft, and I've opted not to use any helpful packages like Tokio yet, in order to get more hands on experience at a lower level.

In the near future I plan to reorganize some of the code and probably rethink the API, since its current form was cobbled together through trial and error, without much thought for things like naming for example. Some other things I'm hoping to implement include:

- Better error variants (+ error messages)
- Consistent error responses for clients
- Unit tests
- More of the essential commands (e.g. `DEL`)
- Runtime config (e.g. max value size)
- Expanded event loop and client management
- Better storage (it's just a hash map right now)

## Usage
At the moment, you can just clone the repo and use `cargo run`, or build the project and run the resulting executable.

## Supported Commands

At the moment, the following commands are supported:

- `PING`
- `ECHO`
- `GET`
- `SET`

The server expects that each command will be serialized in [RESP](https://redis.io/docs/latest/develop/reference/protocol-spec/) format, and sends responses in that format as well. Examples of the request and response formats for each of the supported commands are shown below.

### `PING`

`PING` requests can be made with a [simple string](https://redis.io/docs/latest/develop/reference/protocol-spec/#simple-strings) containing the word `PING`.

The server will respond with a simple string containing the word `PONG` on success.

#### Request
```
+PING\r\n
```

#### Response
```
+PONG\r\n
```

### `ECHO`

Takes an array with two [bulk string](https://redis.io/docs/latest/develop/reference/protocol-spec/#bulk-strings) values, containing the command name and the string that the server should repeat.

#### Request
```
*2\r\n$4\r\nECHO\r\n$12\r\nHello world!\r\n
```

#### Response
```
$12\r\nHello world!\r\n
```

### `GET`

Takes an array with two bulk string values, containing the command name and the key being requested.

If no such key exists in the store, the server responds with a null/empty bulk string; otherwise, returns a bulk string that contains the value stored at that key.

#### Request
```
*2\r\n$3\r\nGET\r\n$8\r\ntest-key
```

#### Response
When key does not exist:

```
$-1\r\n
```

When key does exist with value "foo":

```
$3\r\nfoo\r\n
```

### `SET`

Takes an array with three bulk string values, containing:

1. The command name
2. The key to set
3. The value to set for that key

If a key already exists with the given name, it will be overwritten.

Responds with a simple string containing the word `OK` on success.

#### Request
```
*3\r\n$3\r\nSET\r\n$8\r\ntest-key\r\n$3\r\nfoo\r\n
```

#### Response
```
+OK\r\n
```