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
use thinlinelib::language_type::{Python, C};
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
    let mut thinline: Thinline<Python> = Thinline::new();

    // Reads the source directory where file traversing should start.
    let source_directory = matches
        .value_of("SOURCE-DIR")
        .ok_or_else(|| "CLI parameter 'source_directory' missing.")?;

    // Reads the project config.
    let thinline_cfg_name = matches
        .value_of("project_config")
        .ok_or_else(|| "CLI parameter 'project_config' missing.")?;

    thinline.parse_project_config(source_directory, thinline_cfg_name)?;
    thinline.analyze_project(source_directory)?;

    Ok(())
}
