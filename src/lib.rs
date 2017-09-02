extern crate futures;
extern crate tokio_core;

use tokio_core::reactor::Core;
use futures::Future;

use std::cell::RefCell;

thread_local! {
    static CORE: RefCell<Core> = RefCell::new(Core::new().unwrap());
}

pub fn run_scoped<I, E, F: Future<Item = I, Error = E>>(f: F) -> Result<I, E> {
    CORE.with(|k| {
        let mut core = k.borrow_mut();
        core.run(f)
    })
}

pub fn run<F: 'static + Future<Item = (), Error = ()>>(f: F) {
    CORE.with(|k| {
        let core = k.borrow_mut();
        let handle = core.handle();
        handle.spawn(f);
    });
}
