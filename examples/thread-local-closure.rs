//! Example of logger closure, returning thread-local value
#[macro_use]
extern crate slog;

use std::cell::RefCell;
use std::thread;

use slog::Logger;
use slog::RecordInfo;

thread_local!(static TL_THREAD_ID: RefCell<String> = RefCell::new("main".into()));

fn foo(log: Logger) {
    info!(log, "foo called");
}

fn main() {
    let root = Logger::new_root(o!("thread-id" => |_:&RecordInfo| {
            TL_THREAD_ID.with(|id| { id.borrow().clone() })
        }
        ),
                                slog::drain::discard());

    let mut join = vec![];

    for i in 0..4 {
        join.push(thread::spawn({
            let log = root.clone();
            move || {

                TL_THREAD_ID.with(|id| {
                    *id.borrow_mut() = format!("thread{}", i);
                });

                foo(log);
            }
        }));
    }

    foo(root);

    for join in join {
        join.join().unwrap();
    }
}
