### 0.2.0
- Adds an abstraction for the data store, and some logic to implement key expiration
- Adds a single worker thread to handle operations against the store, and reworks some of the event loop to reflect this change
- Updates `SET` to support the `EX` argument (may expand on this to support the other forms later, e.g. `PX` or `EXAT`)
- Changes `RespSetCommand::value` (and a couple others) from a vector to a boxed slice

### 0.1.0
- Initial release; includes a rough draft of the server event loop and RESP parsing
- Supports the following commands:
  - `PING`
  - `ECHO`
  - `GET`
  - `SET` <sup>[1]</sup>

<sub><sup>[1]</sup> Only supports a basic version of `SET` in this version, without the ability to specify expiry times.</sub>