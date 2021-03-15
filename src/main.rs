#![deny(unused_must_use)]

use sfwtools::run_app;
use std::env;

use contappend::cappend_app;

fn main() {
    let app_name: String = String::from("sfwtools");
    run_app(cappend_app(), env::args().collect(), &app_name)
}
