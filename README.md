# coro

This crate tries to be a bridge between sync and async Rust.
Using tokio's Core from a sync code adds a lot of complexity to the
code.

This crate automatically initializes a thread local event loop and it
exposes only three functions:
* `run` waits for a future to complete and returns its result (like tokio Core's run method),
* `go` spawns a future on the event loop and immediately returns (like tokio Handler's spawn method)
* `handler` acquires a handler to the event loop. It's mostly needed to create the first Future (like a TcpStream).

The function names are very short words, so you can just call
`coro::run(f)` or `coro::go(f)`.

Check the source, it's almost smaller than this README.

It'd be nice to remove the `handler` function and to use the standard library's
I/O types for async as well.

That's all. Happy coding!