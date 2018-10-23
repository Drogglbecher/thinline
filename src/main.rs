extern crate clang;
#[macro_use]
extern crate clap;
extern crate env_logger;
extern crate failure;
extern crate glob;
#[macro_use]
extern crate log;
extern crate thinlinelib;

use clap::App;
use failure::{err_msg, Fallible};
use std::{env::set_var, path::Path, process::exit};
use thinlinelib::{
    language_type::{Cpp, Python, C}, Thinline,
};

fn main() {
    if let Err(res) = run() {
        error!("{}", res);
        exit(1);
    }
}

fn run() -> Fallible<()> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).version(crate_version!());
    let matches = app.get_matches();

    // Sets up logging depending on verbosity level and quiet flag
    let quiet = matches.is_present("quiet");

    if !quiet {
        let lib_log_level = match matches.occurrences_of("verbose") {
            0 => "info",
            1 => "debug",
            _ => "trace",
        };
        set_var(
            "RUST_LOG",
            format!("thinline=warn,thinlinelib={}", lib_log_level),
        );
        env_logger::init();
    }

    // Reads the source directory where file traversing should start.
    let source_directory = matches
        .value_of("SOURCE-DIR")
        .ok_or_else(|| err_msg("CLI parameter 'source_directory' missing."))?;

    // Reads the project config.
    let thinline_cfg_name = matches
        .value_of("project_config")
        .ok_or_else(|| err_msg("CLI parameter 'project_config' missing."))?;

    let language = matches
        .value_of("language")
        .ok_or_else(|| err_msg("CLI parameter 'language' missing."))?;

    let build = matches.is_present("build");

    // Creates a new Thinline instance
    match language {
        "c" => {
            let mut thinline: Thinline<C> = Thinline::new(source_directory);
            thinline.analyze(thinline_cfg_name, build)?;
        }
        "cpp" => {
            let mut thinline: Thinline<Cpp> = Thinline::new(source_directory);
            thinline.analyze(thinline_cfg_name, build)?;
            thinline.synthesize(Path::new("stubs").join("environment"))?;
        }
        "python" => {
            let mut thinline: Thinline<Python> = Thinline::new(source_directory);
            thinline.analyze(thinline_cfg_name, build)?;
        }
        _ => {}
    };

    Ok(())
}
