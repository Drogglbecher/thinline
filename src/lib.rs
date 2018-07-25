#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate clang;
#[macro_use]
extern crate error_chain;
extern crate glob;
extern crate lazy_static;
#[macro_use]
extern crate mac;
extern crate python_parser;
extern crate regex;
extern crate slog_envlogger;
extern crate walkdir;

pub mod analysis;
pub mod argument;
pub mod c;
pub mod error;
pub mod function;
pub mod language_type;
pub mod project_file;
pub mod python;
pub mod synthesis;

use analysis::Analysis;
use error::*;
use language_type::LanguageType;
use std::path::PathBuf;
use synthesis::*;

#[derive(Default, Debug)]
/// Global structure representing the Thinline lib.
pub struct Thinline<T>
where
    T: LanguageType,
{
    /// The structure holding the analysis_c data.
    pub analysis: Analysis<T>,

    /// The structure holding the synthesized testdata.
    pub synthesis: Synthesis,
}

impl<T> Thinline<T>
where
    T: LanguageType,
{
    /// Creates an instance of the lib containing Thinlines functionality.
    pub fn new() -> Self {
        Self::default()
    }

    /// Analyzes the project which should be tested.
    pub fn analyze_project<P: Into<PathBuf>>(&mut self, project_dir: P) -> Result<()> {
        self.analysis = Analysis::new();
        self.analysis
            .collect_sources(&project_dir.into(), &[".", "include"])?;
        self.analysis.extract_entities()?;

        Ok(())
    }
}
