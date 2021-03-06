#[macro_use]
extern crate slog;
extern crate slog_json;
extern crate slog_term;

use slog::*;
use slog::drain::{IntoLogger};

const VERSION: &'static str = "0.1.0";

fn main() {

    let drain = slog_term::async_stderr();
    let log = drain.into_logger(o!("version" => VERSION, "build-id" => "8dfljdf"));

    let log = log.new(o!("owned-fast-lazy" => {
        PushLazy(move |info : &RecordInfo, ser : ValueSerializer| {
            // no need for new allocations
            ser.serialize(info.file())
        })
    }));

    debug!(log, "debug", "fast-lazy" =>
           PushLazy(move |info : &RecordInfo, ser : ValueSerializer| {
               // no need for new allocations
               ser.serialize(&*info.msg())
           })
    );

    trace!(log, "debug", "drop-fast-lazy" =>
        PushLazy(move |_ : &RecordInfo, _ : ValueSerializer| {
            // drop of `ser` will emit unit (`()`/`void`) value
            Ok(())
        })
    );

}
