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
use log::LogLevel;
use std::process::exit;
use thinlinelib::Thinline;
use thinlinelib::error::*;

fn main() {
    if let Err(error) = run() {
        error!("{}", error);
        exit(1);
    }
}

fn run() -> Result<()> {
    // Parse the given arguments
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).version(crate_version!());
    let matches = app.get_matches();

    // Set the verbosity level
    let log_level = match matches.occurrences_of("verbose") {
        0 => LogLevel::Info, // Default value
        1 => LogLevel::Debug,
        _ => LogLevel::Trace,
    };

    let mut thinline = Thinline::new();
    thinline.init_logging(log_level)?;
    thinline.init_environment()?;

    let source_directory = matches.value_of("SOURCE-DIR").ok_or_else(
        || "CLI parameter 'source_directory' missing.",
    )?;

    let thinline_cfg_name = matches.value_of("project_config").ok_or_else(
        || "CLI parameter 'project_config' missing.",
    )?;

    thinline.collect_sources(
        source_directory,
        thinline_cfg_name,
    )?;
    thinline.extract_fct_symbols()?;

    let build = matches.is_present("build");
    if build {
        thinline.execute_build_steps(source_directory)?;
    }

    thinline.synthesize_general_header(source_directory)?;
    thinline.synthesize_testcases(source_directory)?;
    thinline.synthesize_testfiles()?;
    //thinline.reconstruct_fn()?;

    let dry_run = matches.is_present("dry_run");
    if !dry_run {
        thinline.synthesize_gpp_args(source_directory)?;
    }

    Ok(())
}
