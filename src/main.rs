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
use thinlinelib::language_type::{C, Cpp, Python};
use thinlinelib::Thinline;

macro_rules! run {
    ($t:ident, $s: ident, $c: ident) => {
        // Parses the project config.
        $t.parse_project_config($s,$c)?;

        // Analyze the project at the given source directory.
        $t.analyze_project($s)?;
    };
}

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

    // Reads the source directory where file traversing should start.
    let source_directory = matches.value_of("SOURCE-DIR").ok_or_else(
        || "CLI parameter 'source_directory' missing.",
    )?;

    // Reads the project config.
    let thinline_cfg_name = matches.value_of("project_config").ok_or_else(
        || "CLI parameter 'project_config' missing.",
    )?;

    let language = matches.value_of("language").ok_or_else(
        || "CLI parameter 'language' missing.",
    )?;

    // Creates a new Thinline instance
    match language {
        "c" => {
            let mut thinline: Thinline<C> = Thinline::new();
            run!(thinline, source_directory, thinline_cfg_name);
        }
        "cpp" => {
            let mut thinline: Thinline<Cpp> = Thinline::new();
            run!(thinline, source_directory, thinline_cfg_name);
        }
        "python" => {
            let mut thinline: Thinline<Python> = Thinline::new();
            run!(thinline, source_directory, thinline_cfg_name);
        }
        _ => {}
    };

    Ok(())
}
