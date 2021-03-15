//! # cont-append
//! Continuous data appender; can be used for safe log retention.
//!
//!
//! ## Algorithm Overview
//!
//! 1. Given a directory
//! 2. Watch for events recursively in the directory
//! 3.
//!   1. If the event is `Create`, (re)initiate tail on the file.
//!      That is, if we're already tailing, kill the old tail and
//!      start a new one.
//!      `TODO`: (Is this necessary after fixing tail)?
//!   2. If there is a `Delete`, stop the `tail`.
//!   3. Otherwise, presmuably we have appended data to the file,
//!      and let `tail` do its thing.

#![deny(unused_must_use)]
// #![feature(try_trait)]

use std::io::Error;

use seahorse::{App, Command, Context};
use sfwtools::error::*;

pub fn cappend_app() -> App {
    App::new("cappend")
        .author("Brandon Elam Barker")
        .action(run_cappend_seahorse_action)
        .command(run_cappend_seahorse_cmd())
}

const CAPPEND_USAGE: &str = "cappend [SOURCE_FILE] [DEST_FILE]";

pub fn run_cappend_seahorse_cmd() -> Command {
    Command::new("cappend")
        .description(
            "cappend: continuously append or copy
                      files from one directory to another",
        )
        .usage(CAPPEND_USAGE)
        .action(run_cappend_seahorse_action)
}

pub fn run_cappend_seahorse_action(ctxt: &Context) {
    let args = &mut ctxt.args.iter();
    let src = args.next().user_err("cappend: missing source directory");
    let dst = args
        .next()
        .user_err("cappend: missing destination directory");
    run_cappend(&src, &dst);
}

/// Convenience function for running cappend in idiomatic fashion
/// (i.e.) errors are printed to user and the program exits.
pub fn run_cappend(src: &str, dst: &str) {
    cappend(src, dst).user_err("Error in cappend");
}

pub fn cappend(src: &str, dst: &str) -> Result<(), Error> {
    println!("Hello from cappend");
    Ok(())
}
