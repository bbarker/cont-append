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

use anyhow::{Context as ErrContext, Result};

use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use path_abs::PathDir;
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

pub fn cappend(src: &str, dst: &str) -> Result<()> {
    println!("Hello from cappend");
    let mut watcher: RecommendedWatcher = Watcher::new_immediate(|res| {
        println!("Debug: received an event");
        match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    })?;
    watcher
        .configure(Config::PreciseEvents(true))
        .context("Error configuring watcher for PreciseEvents")?;

    let src = PathDir::create(src)?;
    watch_dir(&mut watcher, &src)?;
    Ok(())
}

fn watch_dir(watcher: &mut RecommendedWatcher, dir: &PathDir) -> notify::Result<()> {
    watcher.watch(dir.as_path(), RecursiveMode::Recursive)?;
    Ok(())
}
