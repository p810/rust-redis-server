# Command Reference
As of v0.2.0, the following commands are supported:

- `PING`
- `ECHO`
- `GET`
- `SET`

The server expects that each command will be serialized in [RESP](https://redis.io/docs/latest/develop/reference/protocol-spec/) format, and sends responses in that format as well. Examples of the request and response formats for each of the supported commands are shown below.

## `PING`
```
PING
```

`PING` requests can be made with a [simple string](https://redis.io/docs/latest/develop/reference/protocol-spec/#simple-strings) containing the word `PING`.

The server will respond with a simple string containing the word `PONG` on success.

### Request
```
+PING\r\n
```

### Response
```
+PONG\r\n
```

## `ECHO`
```
ECHO message
```

Takes an array with two [bulk string](https://redis.io/docs/latest/develop/reference/protocol-spec/#bulk-strings) values, containing the command name and the string that the server should repeat.

### Request
```
*2\r\n$4\r\nECHO\r\n$12\r\nHello world!\r\n
```

### Response
```
$12\r\nHello world!\r\n
```

## `GET`
```
GET key
```

Takes an array with two bulk string values, containing the command name and the key being requested.

If no such key exists in the store, the server responds with a null/empty bulk string; otherwise, returns a bulk string that contains the value stored at that key.

### Request
```
*2\r\n$3\r\nGET\r\n$8\r\ntest-key
```

### Response
When key does not exist or has expired:

```
$-1\r\n
```

When key does exist with value "foo":

```
$3\r\nfoo\r\n
```

## `SET`
```
SET key value [EX seconds]
```

Takes an array with at least three bulk string values, containing:

1. The command name
2. The key to set
3. The value to set for that key

Can optionally take two additional values to specify the key's TTL (in seconds):

4. A bulk string with the option name, `EX`, and
5. A positive [integer value](https://redis.io/docs/latest/develop/reference/protocol-spec/#integers) that's greater than zero

If a key already exists with the given name, it will be overwritten.

Responds with a simple string containing the word `OK` on success.

### Request
Setting a key that does not expire:

```
*3\r\n$3\r\nSET\r\n$8\r\ntest-key\r\n$3\r\nfoo\r\n
```

Setting a key that will expire after two minutes:

```
*5\r\n$3\r\nSET\r\n$8\r\ntest-key\r\n$3\r\nfoo\r\n$2\r\nEX\r\n:120\r\n
```

### Response
```
+OK\r\n
```