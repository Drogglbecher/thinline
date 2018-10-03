extern crate clang;
#[macro_use]
extern crate clap;
extern crate failure;
extern crate glob;
extern crate thinlinelib;

use clap::App;
use failure::{err_msg, Fallible};
use thinlinelib::{
    language_type::{Cpp, Python, C},
    Thinline,
};

macro_rules! run {
    ($t:ident, $s:ident, $c:ident) => {
        // Parses the project config.
        $t.parse_project_config($s, $c)?;

        // Analyze the project at the given source directory.
        $t.analyze_project($s)?;
    };
}

fn main() -> Fallible<()> {
    let yaml = load_yaml!("cli.yml");
    let app = App::from_yaml(yaml).version(crate_version!());
    let matches = app.get_matches();

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
