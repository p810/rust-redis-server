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

## Changelog
A list of changes for each tagged version can be found in [`changes.md`](./changes.md).

## Reference
Refer to [`docs/commands.md`](./docs/commands.md) for a list of the commands that the server supports and their arguments, as well as examples of what payloads the server expects for them.