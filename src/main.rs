extern crate clang;
#[macro_use]
extern crate clap;
#[macro_use]
extern crate error_chain;
extern crate glob;
#[macro_use]
extern crate log;
extern crate thinlinelib;

pub mod error;

use clap::App;
use std::process::exit;
use thinlinelib::error::*;
use thinlinelib::Thinline;

fn main() {
    if let Err(error) = run() {
        error!("{}", error);
        exit(1);
    }
}

fn run() -> Result<()> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).version(crate_version!());
    let matches = app.get_matches();

    // Creates a new Thinline instance
    let mut thinline = Thinline::new();

    // Reads the source directory where file traversing should start
    let source_directory = matches
        .value_of("SOURCE-DIR")
        .ok_or_else(|| "CLI parameter 'source_directory' missing.")?;

    thinline.analyze_project(source_directory)?;

    Ok(())
}
