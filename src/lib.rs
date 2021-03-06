extern crate futures;
extern crate tokio_core;

use tokio_core::reactor::{Core, Handle};
use futures::Future;

use std::cell::RefCell;

thread_local! {
    static CORE: RefCell<Core> = RefCell::new(Core::new().unwrap());
}

// Run the f Future on this thread's event loop and wait for its completion, then
// return with its result.
pub fn run<I, E, F: Future<Item = I, Error = E>>(f: F) -> Result<I, E> {
    CORE.with(|k| {
        let mut core = k.borrow_mut();
        core.run(f)
    })
}

// Go runs the f Future on this thread's event loop and immediately returns
// back to the caller.
pub fn go<F: 'static + Future<Item = (), Error = ()>>(f: F) {
    CORE.with(|k| {
        let core = k.borrow_mut();
        let handle = core.handle();
        handle.spawn(f);
    });
}

// Handler returns a handler to this handler's event loop.
pub fn handler() -> Handle {
    CORE.with(|k| k.borrow().handle())
}
