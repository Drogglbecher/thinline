#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]

extern crate clang;
#[macro_use]
extern crate error_chain;
extern crate glob;
extern crate lazy_static;
#[macro_use]
extern crate mac;
extern crate regex;
extern crate slog_envlogger;
extern crate walkdir;

pub mod analysis;
pub mod argument;
pub mod c;
pub mod error;
pub mod function;
pub mod project_file;
pub mod synthesis;

use analysis::Analysis;
use c::analysis_c::AnalysisC;
use error::*;
use std::path::PathBuf;
use synthesis::*;

#[derive(Default)]
/// Global structure representing the Thinline lib.
pub struct Thinline {
    /// The structure holding the analysis_c data.
    pub analysis_c: AnalysisC,

    /// The structure holding the synthesized testdata.
    pub synthesis: Synthesis,
}

impl Thinline {
    /// Creates an instance of the lib containing Thinlines functionality.
    pub fn new() -> Self {
        Self::default()
    }

    /// Analyzes the project which should be tested.
    pub fn analyze_project<P: Into<PathBuf>>(&mut self, project_dir: P) -> Result<()> {
        self.analysis_c = AnalysisC::new();
        self.analysis_c.collect_sources(
            &project_dir.into(),
            &["src", "include"],
        )?;
        self.analysis_c.extract_entities()?;

        Ok(())
    }
}
